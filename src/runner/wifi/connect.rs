use super::utils::*;

use crate::check_known_identifiers::KnownIdentifiers;
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
        let selected_network = if let Some(essid) = self.get_given_essid() {
            get_network_from_given_essid(self, &essid)
        } else {
            get_selected_network(self)
        }?;

        let maybe_key = possibly_get_encryption_key(self, &selected_network)?;
        possibly_configure_network(self, &selected_network, &maybe_key)?;
        connect_to_network(self, &selected_network, &maybe_key)?;
        Ok(())
    }
}

impl WifiDataGatherer for WifiConnectOptions {
    fn get_wifi_data(
        &self,
        synchronous_rescan: &Option<SynchronousRescanType>,
    ) -> Result<(KnownIdentifiers, ScanResult), RuwiError> {
        gather_wifi_network_data(self, synchronous_rescan)
    }
}
