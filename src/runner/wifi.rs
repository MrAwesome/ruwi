use std::thread;

use crate::annotate_networks::annotate_networks;
use crate::configure_network::possibly_configure_network;
use crate::connect::connect_to_network;
use crate::encryption_key::possibly_get_encryption_key;
use crate::find_known_network_names::find_known_network_names;
use crate::options::interfaces::*;
use crate::options::structs::RuwiCommand; // TODO: Remove
use crate::parse::parse_result;
use crate::rerr;
use crate::runner::RuwiStep;
use crate::select_network::select_network;
use crate::sort_networks::sort_and_filter_networks;
use crate::structs::*;
use crate::synchronous_retry_logic::should_retry_with_synchronous_scan;
use crate::wifi_scan::wifi_scan;

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

impl<O: Global + Wifi + WifiConnect + LinuxNetworkingInterface + Clone + Send + Sync> RuwiStep<O>
    for WifiStep
{
    fn exec(self, command: &RuwiCommand, options: &'static O) -> Result<Self, RuwiError> {
        wifi_exec(self, command, options)
    }
}

fn wifi_exec<O>(
    step: WifiStep,
    _command: &RuwiCommand,
    options: &'static O,
) -> Result<WifiStep, RuwiError>
where
    O: Global + Wifi + WifiConnect + LinuxNetworkingInterface + Clone + Send + Sync,
{
    match step {
        WifiStep::ConnectionInit => {
            if let Some(essid) = options.get_given_essid() {
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
            let (known_network_names, scan_result) = gather_wifi_network_data(options, None)?;
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
                gather_wifi_network_data(options, Some(rescan_type))?;
            Ok(WifiStep::NetworkParserAndAnnotator {
                scan_result,
                known_network_names,
            })
        }

        WifiStep::NetworkSorter { annotated_networks } => {
            let sorted_networks = sort_and_filter_networks(options, annotated_networks);
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

fn get_network_from_given_essid<O>(
    options: &O,
    essid: &str,
) -> Result<AnnotatedWirelessNetwork, RuwiError>
where
    O: Global + Wifi + WifiConnect + LinuxNetworkingInterface,
{
    let is_known = find_known_network_names(options)?.contains(essid);
    let is_encrypted = options.get_given_encryption_key().is_some();
    Ok(AnnotatedWirelessNetwork::from_essid(
        essid.into(),
        is_known,
        is_encrypted,
    ))
}

fn gather_wifi_network_data<O>(
    options: &'static O,
    synchronous_rescan: Option<SynchronousRescanType>,
) -> Result<(KnownNetworkNames, ScanResult), RuwiError>
where
    O: Global + Wifi + WifiConnect + LinuxNetworkingInterface + Clone + Send + Sync,
{
    let get_nw_names = thread::spawn(move || find_known_network_names(options));
    let get_scan_results = thread::spawn(move || wifi_scan(options, &synchronous_rescan));

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
    use crate::options::structs::*;

    #[test]
    fn test_basic_runner_functionality() {
        let test_step = WifiStep::BasicTestStep;
        let options = Box::leak(Box::new(WifiConnectOptions::default()));
        let command = RuwiCommand::Wifi(RuwiWifiCommand::Connect(options.clone()));
        let next = test_step.exec(&command, options);
        if let Ok(WifiStep::ConnectionSuccessful) = next {
        } else {
            dbg!(&next);
            panic!("Incorrect return value from basic test step.");
        }
    }

    #[test]
    fn test_connection_init() {
        let options = Box::leak(Box::new(WifiConnectOptions::default()));
        let command = RuwiCommand::Wifi(RuwiWifiCommand::Connect(options.clone()));
        let step = WifiStep::ConnectionInit;
        let expected_next_step = WifiStep::DataGatherer;
        assert_eq!(step.exec(&command, options).unwrap(), expected_next_step);
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
        let options = Box::leak(Box::new(WifiConnectOptions::default()));
        let command = RuwiCommand::Wifi(RuwiWifiCommand::Connect(options.clone()));
        let next = test_step.exec(&command, options);
        if let Ok(WifiStep::NetworkSelector { sorted_networks }) = next {
            assert_eq![first, sorted_networks.networks.first().unwrap().clone()];
            assert_eq![networks.len(), sorted_networks.networks.len()];
        } else {
            dbg!(&next);
            panic!("Next step after default sort wasn't selector.");
        }
    }
}
