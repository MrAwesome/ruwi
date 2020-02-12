use crate::annotate_networks::annotate_networks;
use crate::check_known_identifiers::KnownIdentifiers;
use crate::errors::*;
use crate::parse::parse_result;
use crate::sort_networks::SortedFilteredNetworks;
use crate::structs::*;
use crate::synchronous_retry_logic::should_retry_with_synchronous_scan;
use crate::wifi_scan::wifi_scan;

use crate::runner::Runner;

use crate::options::wifi::select::WifiSelectOptions;

impl Runner for WifiSelectOptions {
    fn run(&self) -> Result<(), RuwiError> {
        self.data_gatherer()
    }
}

impl WifiSelectOptions {
    // TODO: decide if there should be an explicit service management step,
    //       or if services should be managed as they are used for scan/connect/etc
    //       Should you use the service of connect_via? of scan?
    //       It is probably best to have a utility function to start a given service, then
    //       run that as needed whenever a service might be needed.
    fn data_gatherer(&self) -> Result<(), RuwiError> {
        let scan_result = wifi_scan(self, &None)?;
        self.network_parser_and_annotator(&KnownIdentifiers::default(), &scan_result)
    }

    fn network_parser_and_annotator(
        &self,
        known_network_names: &KnownIdentifiers,
        scan_result: &ScanResult,
    ) -> Result<(), RuwiError> {
        let parse_results = parse_result(self, &scan_result)?;
        let annotated_networks =
            annotate_networks(self, &parse_results.seen_networks, &known_network_names);
        if should_retry_with_synchronous_scan(self, &annotated_networks) {
            self.synchronous_rescan(SynchronousRescanType::Automatic)
        } else {
            self.network_sorter(annotated_networks)
        }
    }

    fn synchronous_rescan(&self, rescan_type: SynchronousRescanType) -> Result<(), RuwiError> {
        let scan_result = wifi_scan(self, &Some(rescan_type))?;
        self.network_parser_and_annotator(&KnownIdentifiers::default(), &scan_result)
    }

    fn network_sorter(&self, annotated_networks: Vec<AnnotatedWirelessNetwork>) -> Result<(), RuwiError> {
        let sorted_networks = SortedFilteredNetworks::new(annotated_networks);
        self.network_selector(&sorted_networks)
    }

    fn network_selector(&self, sorted_networks: &SortedFilteredNetworks<AnnotatedWirelessNetwork>) -> Result<(), RuwiError> {
        match sorted_networks.select_network(self) {
            Ok(selected_network) => print_network(&selected_network),
            Err(err) => match &err.kind {
                RuwiErrorKind::RefreshRequested => {
                    self.synchronous_rescan(SynchronousRescanType::ManuallyRequested)
                }
                _ => Err(err),
            },
        }
    }
}

fn print_network(selected_network: &AnnotatedWirelessNetwork) -> Result<(), RuwiError> {
    println!("{}", selected_network.essid);
    Ok(())
}

#[cfg(test)]
mod tests {}
