use crate::annotate_networks::annotate_networks;
use crate::gather_wifi_network_data;
use crate::parse::parse_result;
use crate::should_retry_with_synchronous_scan;
use crate::sort_networks::sort_and_filter_networks;
use crate::structs::*;
use crate::wifi_scan;

#[derive(Debug)]
pub(crate) enum RuwiStep {
    Init,
    WifiDataGatherer,
    WifiNetworkParserAndAnnotator {
        scan_result: ScanResult,
        known_network_names: KnownNetworkNames,
    },
    WifiSynchronousRescan {
        known_network_names: KnownNetworkNames,
    },
    WifiNetworkSorter {
        annotated_networks: AnnotatedNetworks,
    },
    WifiNetworkSelector {
        sorted_networks: SortedUniqueNetworks,
    },
    WifiNetworkConfigurator {
        selected_network: WirelessNetwork,
    },
    WifiNetworkConnector {
        selected_network: WirelessNetwork,
    },
    WifiNetworkConnectionTester,
    #[cfg(test)]
    BasicTestStep,
    Done,
}

impl RuwiStep {
    pub(crate) fn exec(self, command: &RuwiCommand, options: &Options) -> Result<Self, RuwiError> {
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
                        known_network_names,
                    })
                } else {
                    Ok(Self::WifiNetworkSorter { annotated_networks })
                }
            }
            Self::WifiSynchronousRescan {
                known_network_names,
            } => {
                let scan_result =
                    wifi_scan(&options.with_synchronous_retry(SynchronousRetryType::Automatic))?;
                Ok(Self::WifiNetworkParserAndAnnotator {
                    scan_result,
                    known_network_names,
                })
            }
            Self::WifiNetworkSorter { annotated_networks } => {
                let sorted_networks = sort_and_filter_networks(options, annotated_networks.clone());
                Ok(Self::WifiNetworkSelector { sorted_networks })
            }
            //            Self::WifiNetworkSelector { sorted_networks } => {
            //
            //            }
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
}
