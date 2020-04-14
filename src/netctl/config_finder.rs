use super::structs::*;
use typed_builder::TypedBuilder;

// TODO: use predicates? Look for PredicatesStrExt.

pub(super) trait NetctlConfigFinderCriteria {
    type Config: NetctlConfig;

    fn select<'a>(&self, configs: &'a [Self::Config]) -> Vec<&'a Self::Config>;
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub(super) struct WifiNetctlConfigFinderCriteria {
    #[builder(default = None)]
    interface_name: Option<String>,
    #[builder(default = None)]
    identifier_aka_filename: Option<String>,
    #[builder(default = None)]
    essid: Option<String>,
}

impl NetctlConfigFinderCriteria for WifiNetctlConfigFinderCriteria {
    type Config = WifiNetctlConfig;

    fn select<'a>(&self, configs: &'a [Self::Config]) -> Vec<&'a Self::Config> {
        configs
            .iter()
            .filter(|config| {
                (match &self.interface_name {
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
    #[builder(default = None)]
    interface_name: Option<String>,
    #[builder(default = None)]
    identifier_aka_filename: Option<String>,
}

impl NetctlConfigFinderCriteria for WiredNetctlConfigFinderCriteria {
    type Config = WiredNetctlConfig;

    fn select<'a>(&self, configs: &'a [Self::Config]) -> Vec<&'a Self::Config> {
        configs
            .iter()
            .filter(|config| {
                (match &self.interface_name {
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

    fn get_sample_wifi_configs() -> Vec<WifiNetctlConfig> {
        vec![
            WifiNetctlConfig::builder()
                .identifier("MUH_ESSID_IDENT")
                .essid("MUH_ESSID")
                .interface_name("MUH_INTERFACE")
                .encryption_key(None)
                .build(),
            WifiNetctlConfig::builder()
                .identifier("I_AM_SECOND")
                .essid("WHEEEEE")
                .interface_name("wlp60s420")
                .encryption_key(None)
                .build(),
            WifiNetctlConfig::builder()
                .identifier("BRUHBRUH")
                .essid("MUH_ESSID")
                .interface_name("wlp60s420")
                .encryption_key(Some("LOCKED_UP".to_string()))
                .build(),
        ]
    }

    fn get_sample_wired_configs() -> Vec<WiredNetctlConfig> {
        vec![
            WiredNetctlConfig::builder()
                .identifier("MUH_ID")
                .interface_name("BRUH_INTERFACE")
                .build(),
            WiredNetctlConfig::builder()
                .identifier("I_AM_SECOND")
                .interface_name("BRUH_INTERFACE")
                .build(),
            WiredNetctlConfig::builder()
                .identifier("BRUHBRUH")
                .interface_name("enp60s420")
                .build(),
        ]
    }

    #[test]
    fn test_wifi_select_by_interface() {
        let ifname = "MUH_INTERFACE".to_string();
        let configs = get_sample_wifi_configs();
        let criteria = WifiNetctlConfigFinderCriteria::builder()
            .interface_name(Some(ifname.clone()))
            .build();
        let selected_configs = criteria.select(&configs);
        for config in selected_configs {
            assert_eq![ifname, config.interface_name];
        }
    }

    #[test]
    fn test_wired_select_by_interface() {
        let ifname = "BRUH_INTERFACE".to_string();
        let configs = get_sample_wired_configs();
        let criteria = WiredNetctlConfigFinderCriteria::builder()
            .interface_name(Some(ifname.clone()))
            .build();
        let selected_configs = criteria.select(&configs);
        for config in selected_configs {
            assert_eq![ifname, config.interface_name];
        }
    }
}
