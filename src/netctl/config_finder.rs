use super::structs::*;
use typed_builder::TypedBuilder;

// TODO: use predicates? Look for PredicatesStrExt.
// NOTE: when option_result_contains is in stable, the filters below become much simpler: self.interface_name.contains(ifname)

pub(super) trait NetctlConfigFinderCriteria<'a> {
    type Config: NetctlConfig<'a>;

    fn select(&self, configs: Vec<Self::Config>) -> Vec<Self::Config>;
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub(super) struct WifiNetctlConfigFinderCriteria<'a> {
    #[builder(default = None)]
    interface_name: Option<&'a str>,
    #[builder(default = None)]
    identifier_aka_filename: Option<&'a str>,
    #[builder(default = None)]
    essid: Option<&'a str>,
}

pub fn contains<U>(option: &Option<U>, x: &U) -> bool
where
    U: PartialEq,
{
    match option {
        Some(y) => x == y,
        None => false,
    }
}

impl<'a> NetctlConfigFinderCriteria<'a> for WifiNetctlConfigFinderCriteria<'a> {
    type Config = WifiNetctlConfig;

    fn select(&self, configs: Vec<Self::Config>) -> Vec<Self::Config> {
        configs
            .into_iter()
            .filter(|config| {
                contains(&self.interface_name, &config.interface_name.as_ref())
                    && contains(&self.identifier_aka_filename, &config.identifier.as_ref())
                    && contains(&self.essid, &config.essid.as_ref())
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub(super) struct WiredNetctlConfigFinderCriteria<'a> {
    #[builder(default = None)]
    interface_name: Option<&'a str>,
    #[builder(default = None)]
    identifier_aka_filename: Option<&'a str>,
}

impl<'a> NetctlConfigFinderCriteria<'a> for WiredNetctlConfigFinderCriteria<'a> {
    type Config = WiredNetctlConfig;

    fn select(&self, configs: Vec<Self::Config>) -> Vec<Self::Config> {
        configs
            .into_iter()
            .filter(|config| {
                contains(&self.interface_name, &config.interface_name.as_ref())
                    && contains(&self.identifier_aka_filename, &config.identifier.as_ref())
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
        let ifname = "MUH_INTERFACE";
        let configs = get_sample_wifi_configs();
        let criteria = WifiNetctlConfigFinderCriteria::builder()
            .interface_name(Some(ifname))
            .build();
        let selected_configs = criteria.select(configs);
        for config in selected_configs {
            assert_eq![ifname, config.interface_name];
        }
    }

    #[test]
    fn test_wired_select_by_interface() {
        let ifname = "BRUH_INTERFACE";
        let configs = get_sample_wired_configs();
        let criteria = WiredNetctlConfigFinderCriteria::builder()
            .interface_name(Some(ifname))
            .build();
        let selected_configs = criteria.select(configs);
        for config in selected_configs {
            assert_eq![ifname, config.interface_name];
        }
    }
}
