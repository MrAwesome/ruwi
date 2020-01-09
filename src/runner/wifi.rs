use std::thread;

use crate::annotate_networks::annotate_networks;
use crate::configure_network::possibly_configure_network;
use crate::connect::connect_to_network;
use crate::encryption_key::possibly_get_encryption_key;
use crate::find_known_network_names::find_known_network_names;
use crate::parse::parse_result;
use crate::rerr;
use crate::runner::RuwiStep;
use crate::scan::wifi_scan;
use crate::select_network::select_network;
use crate::sort_networks::sort_and_filter_networks;
use crate::structs::*;
use crate::synchronous_retry_logic::should_retry_with_synchronous_scan;

#[derive(Debug, PartialEq)]
pub(crate) enum WifiStep {
    ConnectionInit,
    DataGatherer,
    NetworkParserAndAnnotator {
        scan_result: ScanResult,
        known_network_names: KnownNetworkNames,
    },
    SynchronousRescan {
        rescan_type: SynchronousRescanType,
    },
    NetworkSorter {
        annotated_networks: AnnotatedNetworks,
    },
    NetworkSelector {
        sorted_networks: SortedUniqueNetworks,
    },
    PasswordAsker {
        selected_network: AnnotatedWirelessNetwork,
    },
    NetworkConfigurator {
        selected_network: AnnotatedWirelessNetwork,
        maybe_key: Option<String>,
    },
    NetworkConnector {
        selected_network: AnnotatedWirelessNetwork,
        maybe_key: Option<String>,
    },
    NetworkConnectionTester,
    #[cfg(test)]
    BasicTestStep,
    ConnectionSuccessful,
}

impl RuwiStep for WifiStep {
    fn exec(self, command: &RuwiCommand, options: &Options) -> Result<Self, RuwiError> {
        wifi_exec(self, command, options)
    }
}

fn wifi_exec(
    step: WifiStep,
    _command: &RuwiCommand,
    options: &Options,
) -> Result<WifiStep, RuwiError> {
    match step {
        WifiStep::ConnectionInit => {
            if let Some(essid) = &options.given_essid {
                let selected_network = get_network_from_given_essid(options, &essid)?;
                Ok(WifiStep::PasswordAsker { selected_network })
            } else {
                Ok(WifiStep::DataGatherer)
            }
        }

        // TODO: decide if there should be an explicit service management step,
        //       or if services should be managed as they are used for scan/connect/etc
        //       Should you use the service of connect_via? of scan? 
        //       It is probably best to have a utility function to start a given service, then
        //       run that as needed whenever a service might be needed.
        WifiStep::DataGatherer => {
            let (known_network_names, scan_result) = gather_wifi_network_data(options)?;
            Ok(WifiStep::NetworkParserAndAnnotator {
                known_network_names,
                scan_result,
            })
        }

        WifiStep::NetworkParserAndAnnotator {
            scan_result,
            known_network_names,
        } => {
            let parse_results = parse_result(options, &scan_result)?;
            let annotated_networks =
                annotate_networks(options, &parse_results.seen_networks, &known_network_names);
            if should_retry_with_synchronous_scan(options, &annotated_networks) {
                Ok(WifiStep::SynchronousRescan {
                    rescan_type: SynchronousRescanType::Automatic,
                })
            } else {
                Ok(WifiStep::NetworkSorter { annotated_networks })
            }
        }

        WifiStep::SynchronousRescan { rescan_type } => {
            let (known_network_names, scan_result) =
                gather_wifi_network_data(&options.with_synchronous_retry(rescan_type))?;
            Ok(WifiStep::NetworkParserAndAnnotator {
                scan_result,
                known_network_names,
            })
        }

        WifiStep::NetworkSorter { annotated_networks } => {
            let sorted_networks = sort_and_filter_networks(options, annotated_networks.clone());
            Ok(WifiStep::NetworkSelector { sorted_networks })
        }

        WifiStep::NetworkSelector { sorted_networks } => {
            match select_network(options, &sorted_networks) {
                Ok(selected_network) => Ok(WifiStep::PasswordAsker { selected_network }),
                Err(err) => match &err.kind {
                    RuwiErrorKind::RefreshRequested => Ok(WifiStep::SynchronousRescan {
                        rescan_type: SynchronousRescanType::ManuallyRequested,
                    }),
                    _ => Err(err),
                },
            }
        }

        WifiStep::PasswordAsker { selected_network } => {
            let maybe_key = possibly_get_encryption_key(options, &selected_network)?;
            Ok(WifiStep::NetworkConfigurator {
                selected_network,
                maybe_key,
            })
        }

        WifiStep::NetworkConfigurator {
            selected_network,
            maybe_key,
        } => {
            possibly_configure_network(options, &selected_network, &maybe_key)?;
            Ok(WifiStep::NetworkConnector {
                selected_network,
                maybe_key,
            })
        }

        WifiStep::NetworkConnector {
            selected_network,
            maybe_key,
        } => {
            connect_to_network(options, &selected_network, &maybe_key)?;
            Ok(WifiStep::NetworkConnectionTester)
        }

        WifiStep::NetworkConnectionTester => {
            // TODO: implement
            Ok(WifiStep::ConnectionSuccessful)
        }

        WifiStep::ConnectionSuccessful => term_step(),

        #[cfg(test)]
        WifiStep::BasicTestStep => Ok(WifiStep::ConnectionSuccessful),
    }
}

fn term_step() -> Result<WifiStep, RuwiError> {
    Err(rerr!(
        RuwiErrorKind::UsedTerminalStep,
        "Used terminal step!"
    ))
}

fn get_network_from_given_essid(
    options: &Options,
    essid: &str,
) -> Result<AnnotatedWirelessNetwork, RuwiError> {
    let is_known = find_known_network_names(options)?.contains(essid);
    let is_encrypted = options.given_encryption_key.is_some();
    Ok(AnnotatedWirelessNetwork::from_essid(
        essid.into(),
        is_known,
        is_encrypted,
    ))
}

fn gather_wifi_network_data(
    options: &Options,
) -> Result<(KnownNetworkNames, ScanResult), RuwiError> {
    let (opt1, opt2) = (options.clone(), options.clone());
    let get_nw_names = thread::spawn(move || find_known_network_names(&opt1));
    let get_scan_results = thread::spawn(move || wifi_scan(&opt2));

    let known_network_names = await_thread(get_nw_names)??;
    let scan_result = await_thread(get_scan_results)??;

    Ok((known_network_names, scan_result))
}

#[inline]
fn await_thread<T>(handle: thread::JoinHandle<T>) -> Result<T, RuwiError> {
    handle.join().or_else(|_| {
        Err(rerr!(
            RuwiErrorKind::FailedToSpawnThread,
            "Failed to spawn thread."
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_runner_functionality() {
        let test_step = WifiStep::BasicTestStep;
        let command = RuwiCommand::WifiConnect;
        let options = Options::default();
        let next = test_step.exec(&command, &options);
        if let Ok(WifiStep::ConnectionSuccessful) = next {
        } else {
            dbg!(&next);
            panic!("Incorrect return value from basic test step.");
        }
    }

    #[test]
    fn test_connection_init() {
        let command = RuwiCommand::WifiConnect;
        let options = Options::default();
        let step = WifiStep::ConnectionInit;
        let expected_next_step = WifiStep::DataGatherer;
        assert_eq!(step.exec(&command, &options).unwrap(), expected_next_step);
    }

    //        if let WifiStep::NetworkParserAndAnnotator { .. } = &step {
    //            dbg!(&step);
    //        }
    // TODO: test every one of these transitions
    //    WifiStep::ConnectionInit => Ok(WifiStep::DataGatherer),
    //    WifiStep::DataGatherer => {
    //                Ok(WifiStep::NetworkParserAndAnnotator {
    //                    known_network_names: known_network_names,
    //                    scan_result,
    //                })
    //
    //    WifiStep::NetworkParserAndAnnotator { scan_result, known_network_names, }
    //                    Ok(WifiStep::SynchronousRescan {
    //                        rescan_type: SynchronousRescanType::Automatic,
    //                    })
    //                    Ok(WifiStep::NetworkSorter { annotated_networks })
    //
    //    WifiStep::SynchronousRescan { rescan_type } => {
    //                Ok(WifiStep::NetworkParserAndAnnotator {
    //                    scan_result,
    //                    known_network_names,
    //                })
    //
    //    WifiStep::NetworkSorter { annotated_networks } => {
    //                Ok(WifiStep::NetworkSelector { sorted_networks })
    //
    //    WifiStep::NetworkSelector { sorted_networks } => {
    //                    Ok(WifiStep::PasswordAsker { selected_network }),
    //                    Ok(WifiStep::SynchronousRescan {
    //                        rescan_type: SynchronousRescanType::ManuallyRequested,
    //                    }),
    //    WifiStep::PasswordAsker { selected_network } => {
    //                Ok(WifiStep::NetworkConfigurator { selected_network, maybe_key, })
    //
    //    WifiStep::NetworkConfigurator { selected_network, maybe_key } => {
    //            Ok(WifiStep::NetworkConnector { selected_network, maybe_key})
    //
    //    WifiStep::NetworkConnector {
    //                Ok(WifiStep::NetworkConnectionTester)
    //
    //    WifiStep::NetworkConnectionTester => {
    //                Ok(WifiStep::ConnectionSuccessful)
    //    WifiStep::BasicTestStep => Ok(WifiStep::ConnectionSuccessful),

    #[test]
    fn test_sorter_into_selector() {
        let first = AnnotatedWirelessNetwork::from_essid("I AM FIRST".into(), true, false);
        let second = AnnotatedWirelessNetwork::from_essid("I AM SECOND".into(), false, false);
        let networks = vec![second, first.clone()];
        let annotated_networks = AnnotatedNetworks {
            networks: networks.clone(),
        };
        let test_step = WifiStep::NetworkSorter { annotated_networks };
        let command = RuwiCommand::WifiConnect;
        let options = Options::default();
        let next = test_step.exec(&command, &options);
        if let Ok(WifiStep::NetworkSelector { sorted_networks }) = next {
            assert_eq![first, sorted_networks.networks.first().unwrap().clone()];
            assert_eq![networks.len(), sorted_networks.networks.len()];
        } else {
            dbg!(&next);
            panic!("Next step after default sort wasn't selector.");
        }
    }
}
