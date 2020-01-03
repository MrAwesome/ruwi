// TODO: move these pieces into their own helper library
use crate::gather_wifi_network_data;
use crate::select_network;
use crate::connect_to_network;
use crate::possibly_get_encryption_key;
use crate::should_retry_with_synchronous_scan;
use crate::possibly_configure_network;

use crate::annotate_networks::annotate_networks;
use crate::parse::parse_result;
use crate::sort_networks::sort_and_filter_networks;
use crate::structs::*;

#[derive(Debug)]
pub(crate) enum RuwiStep {
    Init,
    WifiDataGatherer,
    WifiNetworkParserAndAnnotator {
        scan_result: ScanResult,
        known_network_names: KnownNetworkNames,
    },
    WifiSynchronousRescan {
        rescan_type: SynchronousRescanType,
    },
    WifiNetworkSorter {
        annotated_networks: AnnotatedNetworks,
    },
    WifiNetworkSelector {
        sorted_networks: SortedUniqueNetworks,
    },
    WifiNetworkConfigurator {
        selected_network: AnnotatedWirelessNetwork,
        maybe_key: Option<String>,
    },
    WifiNetworkConnector {
        selected_network: AnnotatedWirelessNetwork,
        maybe_key: Option<String>,
    },
    WifiPasswordAsker {
        selected_network: AnnotatedWirelessNetwork,
    },
    WifiNetworkConnectionTester,
    #[cfg(test)]
    BasicTestStep,
    Done,
}

impl RuwiStep {
    pub(crate) fn exec(self, command: &RuwiCommand, options: &Options) -> Result<Self, RuwiError> {
        match command {
            RuwiCommand::WifiConnect => {
                self.wifi_exec(options)
            }
        }
    }

    // TODO: flow for given essid

    fn wifi_exec(self, options: &Options) -> Result<Self, RuwiError> {
        match self {
            Self::Init => Ok(Self::WifiDataGatherer),

            // TODO: decide if there should be an explicit service management step, or if services should be managed as they are used for scan/connect/etc

            Self::WifiDataGatherer => {
                let (known_network_names, scan_result) = gather_wifi_network_data(options)?;
                Ok(Self::WifiNetworkParserAndAnnotator {
                    known_network_names: known_network_names,
                    scan_result,
                })
            }

            Self::WifiNetworkParserAndAnnotator {
                scan_result,
                known_network_names,
            } => {
                let parse_results = parse_result(options, &scan_result)?;
                let annotated_networks =
                    annotate_networks(options, &parse_results.seen_networks, &known_network_names);
                // TODO: implement retry here
                if should_retry_with_synchronous_scan(options, &annotated_networks) {
                    Ok(Self::WifiSynchronousRescan {
                        rescan_type: SynchronousRescanType::Automatic,
                    })
                } else {
                    Ok(Self::WifiNetworkSorter { annotated_networks })
                }
            }

            Self::WifiSynchronousRescan { rescan_type } => {
                let (known_network_names, scan_result) =
                    gather_wifi_network_data(&options.with_synchronous_retry(rescan_type))?;
                Ok(Self::WifiNetworkParserAndAnnotator {
                    scan_result,
                    known_network_names,
                })
            }

            Self::WifiNetworkSorter { annotated_networks } => {
                let sorted_networks = sort_and_filter_networks(options, annotated_networks.clone());
                Ok(Self::WifiNetworkSelector { sorted_networks })
            }

            Self::WifiNetworkSelector { sorted_networks } => {
                match select_network(options, &sorted_networks) {
                    Ok(selected_network) => Ok(Self::WifiPasswordAsker { selected_network }),
                    Err(err) => match &err.kind {
                        RuwiErrorKind::RefreshRequested => Ok(Self::WifiSynchronousRescan {
                            rescan_type: SynchronousRescanType::ManuallyRequested,
                        }),
                        _ => Err(err),
                    },
                }
            }

            Self::WifiPasswordAsker {
                selected_network
            } => {
                let maybe_key = possibly_get_encryption_key(options, &selected_network)?;
                Ok(Self::WifiNetworkConfigurator {
                    selected_network,
                    maybe_key
                })
            }

            Self::WifiNetworkConfigurator {
                selected_network,
                maybe_key
            } => {
                possibly_configure_network(options, &selected_network, &maybe_key)?;
                Ok(Self::WifiNetworkConnector { selected_network, maybe_key })
            }

            Self::WifiNetworkConnector { selected_network, maybe_key } => {
                connect_to_network(options, &selected_network, &maybe_key)?;
                Ok(Self::Done)
            }

            Self::WifiNetworkConnectionTester => {
                // TODO: implement
                Ok(Self::Done)
            }

            #[cfg(test)]
            Self::BasicTestStep => Ok(Self::Done),

            x => {
                dbg!(x);
                panic!("FUCK");
            }

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_runner_functionality() {
        let test_step = RuwiStep::BasicTestStep;
        let command = RuwiCommand::default();
        let options = Options::default();
        let next = test_step.exec(&command, &options);
        if let Ok(RuwiStep::Done) = next {
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
        let test_step = RuwiStep::WifiNetworkSorter { annotated_networks };
        let command = RuwiCommand::default();
        let options = Options::default();
        let next = test_step.exec(&command, &options);
        if let Ok(RuwiStep::WifiNetworkSelector { sorted_networks }) = next {
            assert_eq![first, sorted_networks.networks.first().unwrap().clone()];
            assert_eq![networks.len(), sorted_networks.networks.len()];
        } else {
            dbg!(&next);
            panic!("Next step after default sort wasn't selector.");
        }
    }
}
