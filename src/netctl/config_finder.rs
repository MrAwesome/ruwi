use super::*;
use typed_builder::TypedBuilder;

// TODO: use predicates? Look for PredicatesStrExt.

pub(super) trait NetctlConfigFinderCriteria {
    type Config: NetctlConfig;

    fn select<'a>(&self, configs: &'a [Self::Config]) -> Vec<&'a Self::Config>;
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub(super) struct WifiNetctlConfigFinderCriteria {
    interface: Option<String>,
    identifier_aka_filename: Option<String>,
    essid: Option<String>,
}

impl NetctlConfigFinderCriteria for WifiNetctlConfigFinderCriteria {
    type Config = WifiNetctlConfig;

    fn select<'a>(&self, configs: &'a [Self::Config]) -> Vec<&'a Self::Config> {
        configs
            .iter()
            .filter(|config| {
                (match &self.interface {
                    Some(ifname) => ifname == &config.interface_name,
                    None => true,
                }) && (match &self.identifier_aka_filename {
                    Some(id) => id == config.identifier.as_ref(),
                    None => true,
                }) && (match &self.essid {
                    Some(essid) => essid == &config.essid,
                    None => true,
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub(super) struct WiredNetctlConfigFinderCriteria {
    interface: Option<String>,
    identifier_aka_filename: Option<String>,
}

impl NetctlConfigFinderCriteria for WiredNetctlConfigFinderCriteria {
    type Config = WiredNetctlConfig;

    fn select<'a>(&self, configs: &'a [Self::Config]) -> Vec<&'a Self::Config> {
        configs
            .iter()
            .filter(|config| {
                (match &self.interface {
                    Some(ifname) => ifname == &config.interface_name,
                    None => true,
                }) && (match &self.identifier_aka_filename {
                    Some(id) => id == config.identifier.as_ref(),
                    None => true,
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wifi_select_by_interface() {
        WifiNetctlConfig::builder()
            .identifier("MUH_ESSID_IDENT")
            .essid("MUH_ESSID")
            .interface_name("MUH_INTERFACE")
            .encryption_key(None)
            .build();
    }
}
