use super::utils::*;

use crate::known_networks::WifiKnownNetworks;
use crate::interface_management::ip_interfaces::*;
use crate::configure_network::possibly_configure_network;
use crate::connect::wifi_connect::connect_to_network;
use crate::encryption_key::possibly_get_encryption_key;
use crate::enums::*;
use crate::errors::*;
use crate::options::interfaces::*;
use crate::options::wifi::connect::WifiConnectOptions;
use crate::runner::Runner;
use crate::structs::*;

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
