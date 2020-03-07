use super::utils::*;

use crate::check_known_identifiers::KnownIdentifiers;
use crate::interface_management::ip_interfaces::*;
use crate::enums::*;
use crate::errors::*;
use crate::options::interfaces::*;
use crate::options::wifi::select::WifiSelectOptions;
use crate::runner::Runner;
use crate::structs::*;
use crate::wifi_scan::wifi_scan;

impl Runner for WifiSelectOptions {
    fn run(&self) -> Result<(), RuwiError> {
        let interface = WifiIPInterface::from_name_or_first(self, self.get_given_interface_name())?;
        let selected_network = scan_and_select_network(self, &interface)?;
        println!("{}", selected_network.get_identifier());
        Ok(())
    }
}

impl WifiDataGatherer for WifiSelectOptions {
    fn get_wifi_data(
        &self,
        interface: &WifiIPInterface,
        synchronous_rescan: &Option<SynchronousRescanType>,
    ) -> Result<(KnownIdentifiers, ScanResult), RuwiError> {
        let scan_result = wifi_scan(self, interface, synchronous_rescan)?;
        Ok((KnownIdentifiers::default(), scan_result))
    }
}
