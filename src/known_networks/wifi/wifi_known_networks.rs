use std::collections::HashMap;
use std::iter::FromIterator;

type Essid = String;

// For netctl, this is the filename of the config. For NetworkManager, it is just the essid.
type ServiceIdentifier = String;

pub(super) type UnparsedKnownNetworkNamesAndIdentifiers = Vec<(Essid, ServiceIdentifier)>;

#[derive(Debug, PartialEq, Eq)]
pub struct WifiKnownNetworks {
    essid_to_identifiers: HashMap<Essid, ServiceIdentifier>,
}

impl Default for WifiKnownNetworks {
    fn default() -> Self {
        Self {
            essid_to_identifiers: HashMap::new(),
        }
    }
}

impl WifiKnownNetworks {
    pub(crate) fn new(seen_networks: UnparsedKnownNetworkNamesAndIdentifiers) -> Self {
        let essid_to_identifiers = HashMap::from_iter(seen_networks);
        Self {
            essid_to_identifiers,
        }
    }

//    pub(crate) fn check_for_essid(&self, essid: &str) -> bool {
//        self.essid_to_identifiers.contains_key(essid)
//    }
//
    pub(crate) fn get_service_identifier_for_essid(&self, essid: &str) -> Option<&str> {
        self.essid_to_identifiers.get(essid).map(|x| x.as_ref())
    }
}
