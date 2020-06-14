use super::utils::scan_and_select_network;

use crate::interface_management::ip_interfaces::{LinuxIPInterface, WifiIPInterface};
use crate::known_networks::WifiKnownNetworks;
use crate::options::wifi::select::WifiSelectOptions;
use crate::prelude::*;
use crate::runner::Runner;
use crate::wifi_scan::wifi_scan;

impl Runner for WifiSelectOptions {
    fn run(&self) -> Result<(), RuwiError> {
        let interface = WifiIPInterface::from_name_or_first(self, self.get_given_interface_name())?;
        let selected_network = scan_and_select_network(self, &interface)?;
        println!("{}", selected_network.get_public_name());
        Ok(())
    }
}

impl WifiDataGatherer for WifiSelectOptions {
    fn get_wifi_data(
        &self,
        interface: &WifiIPInterface,
        synchronous_rescan: &Option<SynchronousRescanType>,
    ) -> Result<(WifiKnownNetworks, ScanResult), RuwiError> {
        let scan_result = wifi_scan(self, interface, synchronous_rescan)?;
        Ok((WifiKnownNetworks::default(), scan_result))
    }
}
