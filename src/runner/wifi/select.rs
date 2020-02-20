use super::utils::*;

use crate::check_known_identifiers::KnownIdentifiers;
use crate::enums::*;
use crate::errors::*;
use crate::options::interfaces::*;
use crate::options::wifi::select::WifiSelectOptions;
use crate::runner::Runner;
use crate::structs::*;
use crate::wifi_scan::wifi_scan;

impl Runner for WifiSelectOptions {
    fn run(&self) -> Result<(), RuwiError> {
        let selected_network = get_selected_network(self)?;
        println!("{}", selected_network.get_identifier());
        Ok(())
    }
}

impl WifiDataGatherer for WifiSelectOptions {
    fn get_wifi_data(
        &self,
        synchronous_rescan: &Option<SynchronousRescanType>,
    ) -> Result<(KnownIdentifiers, ScanResult), RuwiError> {
        let scan_result = wifi_scan(self, synchronous_rescan)?;
        Ok((KnownIdentifiers::default(), scan_result))
    }
}
