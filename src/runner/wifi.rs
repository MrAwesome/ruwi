use crate::connect_to_network;
use crate::gather_wifi_network_data;
use crate::possibly_configure_network;
use crate::possibly_get_encryption_key;
use crate::select_network;
use crate::should_retry_with_synchronous_scan;
use crate::runner::RuwiStep;

use crate::annotate_networks::annotate_networks;
use crate::parse::parse_result;
use crate::sort_networks::sort_and_filter_networks;
use crate::structs::*;


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
    command: &RuwiCommand,
    options: &Options,
) -> Result<WifiStep, RuwiError> {
    match step {
        // TODO: Skip ahead when no scan necessary
        WifiStep::ConnectionInit => Ok(WifiStep::DataGatherer),

        // TODO: decide if there should be an explicit service management step, or if services should be managed as they are used for scan/connect/etc
        WifiStep::DataGatherer => {
            let (known_network_names, scan_result) = gather_wifi_network_data(options)?;
            Ok(WifiStep::NetworkParserAndAnnotator {
                known_network_names: known_network_names,
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
            // TODO: implement retry here
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

        #[cfg(test)]
        WifiStep::BasicTestStep => Ok(WifiStep::ConnectionSuccessful),

        x => {
            dbg!(x);
            panic!("FUCK");
        }
    }
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
