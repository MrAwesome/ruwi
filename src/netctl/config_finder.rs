use super::*;
use typed_builder::TypedBuilder;

// TODO: use predicates? PredicatesStrExt is nice.

pub(super) trait NetctlConfigFinderCriteria {
    type Config: NetctlConfig;

    fn select<'a>(&self, configs: &'a [Self::Config]) -> Vec<&'a Self::Config>;
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub(super) struct WifiNetctlConfigFinderCriteria {
    interface: String, // TODO: also make option?
    identifier_aka_filename: Option<String>,
    essid: Option<String>,
}

impl NetctlConfigFinderCriteria for WifiNetctlConfigFinderCriteria {
    type Config = WifiNetctlConfig;

    fn select<'a>(&self, configs: &'a [Self::Config]) -> Vec<&'a Self::Config> {
        configs
            .iter()
            .filter(|config| {
                self.interface == config.interface_name
                    && match &self.identifier_aka_filename {
                        Some(id) => id == config.identifier.as_ref(),
                        None => true,
                    }
                    && match &self.essid {
                        Some(essid) => essid == &config.essid,
                        None => true,
                    }
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub(super) struct WiredNetctlConfigFinderCriteria {
    interface: String,
    identifier_aka_filename: Option<String>,
}

impl NetctlConfigFinderCriteria for WiredNetctlConfigFinderCriteria {
    type Config = WiredNetctlConfig;

    fn select<'a>(&self, configs: &'a [Self::Config]) -> Vec<&'a Self::Config> {
        configs
            .iter()
            .filter(|config| {
                self.interface == config.interface_name
                    && match &self.identifier_aka_filename {
                        Some(id) => id == config.identifier.as_ref(),
                        None => true,
                    }
            })
            .collect()
    }
}
