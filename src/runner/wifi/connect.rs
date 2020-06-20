use super::utils::{
    gather_wifi_network_data, get_network_from_given_essid, scan_and_select_network,
};

use crate::configure_network::possibly_configure_network;
use crate::connect::wifi_connect::connect_to_network;
use crate::encryption_key::possibly_get_encryption_key;
use crate::interface_management::ip_interfaces::{LinuxIPInterface, WifiIPInterface};
use crate::known_networks::WifiKnownNetworks;
use crate::options::wifi::connect::WifiConnectOptions;
use crate::prelude::*;
use crate::runner::Runner;

impl Runner for WifiConnectOptions {
    fn run(&self) -> Result<(), RuwiError> {
        let interface = WifiIPInterface::from_name_or_first(self, self.get_given_interface_name())?;

        let selected_network = if let Some(essid) = self.get_given_essid() {
            get_network_from_given_essid(self, &essid)
        } else {
            scan_and_select_network(self, &interface)
        }?;

        let maybe_key = possibly_get_encryption_key(self, &selected_network)?;
        possibly_configure_network(self, &interface, &selected_network, &maybe_key)?;
        connect_to_network(self, &interface, &selected_network, &maybe_key)?;
        Ok(())
    }
}

impl WifiDataGatherer for WifiConnectOptions {
    fn get_wifi_data(
        &self,
        interface: &WifiIPInterface,
        synchronous_rescan: &Option<SynchronousRescanType>,
    ) -> Result<(WifiKnownNetworks, ScanResult), RuwiError> {
        gather_wifi_network_data(self, interface, synchronous_rescan)
    }
}
