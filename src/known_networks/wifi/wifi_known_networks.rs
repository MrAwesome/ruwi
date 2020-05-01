use crate::enums::NetworkServiceIdentifier;
use std::collections::HashMap;
use std::iter::FromIterator;

type Essid = String;

// For netctl, this is the filename of the config. For NetworkManager, it is just the essid.
pub(super) type UnfilteredKnownNetworkNamesAndIdentifiers = Vec<(Essid, NetworkServiceIdentifier)>;

#[derive(Debug, PartialEq, Eq)]
pub struct WifiKnownNetworks {
    essid_to_identifiers: HashMap<Essid, NetworkServiceIdentifier>,
}

impl Default for WifiKnownNetworks {
    fn default() -> Self {
        Self {
            essid_to_identifiers: HashMap::new(),
        }
    }
}

impl WifiKnownNetworks {
    pub(crate) fn new(seen_networks: UnfilteredKnownNetworkNamesAndIdentifiers) -> Self {
        let essid_to_identifiers = HashMap::from_iter(seen_networks);
        Self {
            essid_to_identifiers,
        }
    }

//    pub(crate) fn check_for_essid(&self, essid: &str) -> bool {
//        self.essid_to_identifiers.contains_key(essid)
//    }
//
    pub(crate) fn get_service_identifier_for_essid(&self, essid: &str) -> Option<&NetworkServiceIdentifier> {
        self.essid_to_identifiers.get(essid)
    }
}
