use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Eq)]
pub struct KnownIdentifiers {
    identifiers: HashSet<String>,
}

impl Default for KnownIdentifiers {
    fn default() -> Self {
        Self {
            identifiers: HashSet::new(),
        }
    }
}

impl KnownIdentifiers {
    pub fn new(seen_networks: Vec<String>) -> Self {
        let unique_networks = HashSet::from_iter(seen_networks);
        Self {
            identifiers: unique_networks,
        }
    }

    pub fn check_for(&self, target: &str) -> bool {
        self.identifiers.contains(target)
    }
}
