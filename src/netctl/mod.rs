pub(crate) mod config_finder;
pub(crate) mod config_parser;
pub(crate) mod config_reader;
pub(crate) mod config_writer;
pub(crate) mod structs;
pub(crate) mod utils;

// TODO: make existing netctl configuration for wifi use handler+reader instead
// TODO: refactor wired connections to use AnnotatedWiredNetwork
// TODO: make netctl wired use this, use selection if more than one wired network profile exists (also for networkmanager)
// TODO: consider consolidating interface+networks

use config_finder::*;
#[cfg(not(test))]
use config_reader::read_all_netctl_config_files;
use structs::*;

use crate::common::*;
use crate::interface_management::ip_interfaces::*;
use std::convert::TryFrom;
use typed_builder::TypedBuilder;

const DEFAULT_NETCTL_CFG_DIR: &str = "/etc/netctl/";

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub(crate) struct NetctlConfigHandler<'a, O: Global> {
    opts: &'a O,
    #[builder(default = DEFAULT_NETCTL_CFG_DIR.to_string())]
    netctl_cfg_dir: String,
    #[cfg(test)]
    #[builder(default)]
    given_raw_configs: Vec<NetctlRawConfig<'a>>,
}

impl<'a, O: Global> NetctlConfigHandler<'a, O> {
    pub(crate) fn new(opts: &'a O) -> Self {
        NetctlConfigHandler::builder().opts(opts).build()
    }

    fn get_netctl_cfg_dir(&self) -> &str {
        &self.netctl_cfg_dir
    }

    #[cfg(test)]
    fn get_all_configs_text(&'a self) -> Result<Vec<NetctlRawConfig<'a>>, RuwiError> {
        Ok(self.given_raw_configs.clone())
    }

    #[cfg(not(test))]
    fn get_all_configs_text(&'a self) -> Result<Vec<NetctlRawConfig<'a>>, RuwiError> {
        read_all_netctl_config_files(self.netctl_cfg_dir.as_ref())
    }

    fn get_all_parsed_but_untyped_configs(&self) -> Result<Vec<NetctlRawParsedFields>, RuwiError> {
        Ok(self
            .get_all_configs_text()?
            .iter()
            .filter_map(|text| NetctlRawParsedFields::try_from(text).ok())
            .collect())
    }

    fn get_all_typed_configs<C>(&self) -> Result<Vec<C>, RuwiError>
    where
        C: NetctlConfig<'a>,
    {
        Ok(self
            .get_all_parsed_but_untyped_configs()?
            .into_iter()
            .filter_map(|config| C::try_from(config).ok())
            .collect::<Vec<C>>())
    }

    fn find_matching_configs<C>(
        &self,
        criteria: &C::Checker,
    ) -> Result<
        Vec<<<C as NetctlConfig<'a>>::Checker as NetctlConfigFinderCriteria<'a>>::Config>,
        RuwiError,
    >
    where
        C: NetctlConfig<'a>,
    {
        let all_typed_configs = self.get_all_typed_configs()?;
        let matching_configs = criteria.select(all_typed_configs);
        Ok(matching_configs)
    }

    // put this into a trait and implement for both kinds of network/interface
    pub(crate) fn write_wifi_config(
        &self,
        interface: &WifiIPInterface,
        network: &AnnotatedWirelessNetwork,
        encryption_key: &Option<String>,
    ) -> Result<ConfigResult, RuwiError> {
        let config = WifiNetctlConfig::new(interface, network, encryption_key);

        self.write_config_to_file(&config)
    }

    // put this into a trait and implement for both kinds of network/interface
    pub(crate) fn write_wired_config(
        &self,
        interface: &WiredIPInterface,
        network: &AnnotatedWiredNetwork,
    ) -> Result<ConfigResult, RuwiError> {
        let config = WiredNetctlConfig::new(interface, network);

        self.write_config_to_file(&config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::options::GlobalOptions;

    pub(super) static ETHERNET_SAMPLE_FILENAME: &str = "ethernet_dhcp";
    pub(super) static WIRELESS_OPEN_SAMPLE_FILENAME: &str = "Chateau_de_Chine_Hotel";
    pub(super) static WIRELESS_ENCRYPTED_SAMPLE_FILENAME: &str = "kingship_lobby";

    pub(super) static ETHERNET_SAMPLE: &str = include_str!("samples/ethernet_dhcp");
    pub(super) static WIRELESS_OPEN_SAMPLE: &str = include_str!("samples/Chateau_de_Chine_Hotel");
    pub(super) static WIRELESS_ENCRYPTED_SAMPLE: &str = include_str!("samples/kingship_lobby");

    fn get_sample_filenames() -> Vec<&'static str> {
        vec![
            ETHERNET_SAMPLE_FILENAME,
            WIRELESS_OPEN_SAMPLE_FILENAME,
            WIRELESS_ENCRYPTED_SAMPLE_FILENAME,
        ]
    }

    fn get_sample_contents(filename: &str) -> &'static str {
        match filename {
            _ if filename == ETHERNET_SAMPLE_FILENAME => ETHERNET_SAMPLE,
            _ if filename == WIRELESS_OPEN_SAMPLE_FILENAME => WIRELESS_OPEN_SAMPLE,
            _ if filename == WIRELESS_ENCRYPTED_SAMPLE_FILENAME => WIRELESS_ENCRYPTED_SAMPLE,
            _ => panic!(format!("Unknown filename {}!", filename)),
        }
    }

    fn get_sample_raw_configs<'a>(location: &'a str) -> Vec<NetctlRawConfig<'a>> {
        let locations = get_sample_filenames();
        locations
            .into_iter()
            .map(|filename| {
                let file_contents = get_sample_contents(filename);
                NetctlRawConfig::builder()
                    .identifier(filename)
                    .contents(file_contents)
                    .location(location)
                    .build()
            })
            .collect()
    }

    fn get_expected_untyped_parsed_config(filename: &str) -> NetctlRawParsedFields {
        match filename {
            _ if filename == ETHERNET_SAMPLE_FILENAME => NetctlRawParsedFields {
                identifier: NetctlIdentifier::new(ETHERNET_SAMPLE_FILENAME),
                connection_type: NetctlConnectionType::Wired,
                interface_name: "enp0s25".to_string(),
                essid: None,
                encryption_key: None,
            },
            _ if filename == WIRELESS_OPEN_SAMPLE_FILENAME => NetctlRawParsedFields {
                identifier: NetctlIdentifier::new(WIRELESS_OPEN_SAMPLE_FILENAME),
                connection_type: NetctlConnectionType::Wifi,
                interface_name: "wlp3s0".to_string(),
                essid: Some("Chateau de Chine Hotel".to_string()),
                encryption_key: None,
            },
            _ if filename == WIRELESS_ENCRYPTED_SAMPLE_FILENAME => NetctlRawParsedFields {
                identifier: NetctlIdentifier::new(WIRELESS_ENCRYPTED_SAMPLE_FILENAME),
                connection_type: NetctlConnectionType::Wifi,
                interface_name: "wlp3s1".to_string(),
                essid: Some("Lobby".to_string()),
                encryption_key: Some("KS211819".to_string()),
            },
            _ => panic!(format!("Config {} not found!", filename)),
        }
    }

    fn get_sample_handler<'a, O: Global>(opts: &'a O) -> NetctlConfigHandler<'a, O> {
        let location = "samples/";
        let sample_configs = get_sample_raw_configs(location);
        NetctlConfigHandler::builder()
            .opts(opts)
            .netctl_cfg_dir(location)
            .given_raw_configs(sample_configs)
            .build()
    }

    fn get_expected_wifi_config(filename: &str) -> WifiNetctlConfig {
        match filename {
            _ if filename == WIRELESS_OPEN_SAMPLE_FILENAME => WifiNetctlConfig {
                identifier: NetctlIdentifier::new(WIRELESS_OPEN_SAMPLE_FILENAME),
                interface_name: "wlp3s0".to_string(),
                essid: "Chateau de Chine Hotel".to_string(),
                encryption_key: None,
            },
            _ if filename == WIRELESS_ENCRYPTED_SAMPLE_FILENAME => WifiNetctlConfig {
                identifier: NetctlIdentifier::new(WIRELESS_ENCRYPTED_SAMPLE_FILENAME),
                interface_name: "wlp3s1".to_string(),
                essid: "Lobby".to_string(),
                encryption_key: Some("KS211819".to_string()),
            },
            _ => panic!(format!("Wifi config {} not found!", filename)),
        }
    }

    fn get_expected_wired_config(filename: &str) -> WiredNetctlConfig {
        match filename {
            _ if filename == ETHERNET_SAMPLE_FILENAME => WiredNetctlConfig {
                identifier: NetctlIdentifier::new(ETHERNET_SAMPLE_FILENAME),
                interface_name: "enp0s25".to_string(),
            },
            _ => panic!(format!("Wired config {} not found!", filename)),
        }
    }

    #[test]
    fn test_find_and_parse_untyped_configs() {
        let opts = GlobalOptions::default();
        let handler = get_sample_handler(&opts);

        let untyped_configs = handler.get_all_parsed_but_untyped_configs().unwrap();
        for filename in get_sample_filenames() {
            let config = untyped_configs
                .iter()
                .find(|&x| x.identifier == *filename)
                .unwrap();
            assert_eq![config, &get_expected_untyped_parsed_config(filename)]
        }
    }

    #[test]
    fn test_find_and_parse_wifi_configs() {
        let opts = GlobalOptions::default();
        let handler = get_sample_handler(&opts);

        let wifi_configs: Vec<WifiNetctlConfig> = handler.get_all_typed_configs().unwrap();
        dbg![&wifi_configs];

        let ethernet_sample_res = wifi_configs
            .iter()
            .find(|&x| x.identifier == *ETHERNET_SAMPLE_FILENAME);
        assert![ethernet_sample_res.is_none()];

        let wireless_encrypted_config = wifi_configs
            .iter()
            .find(|&x| x.identifier == *WIRELESS_ENCRYPTED_SAMPLE_FILENAME)
            .unwrap();
        assert_eq![
            wireless_encrypted_config,
            &get_expected_wifi_config(WIRELESS_ENCRYPTED_SAMPLE_FILENAME)
        ];

        let wireless_open_config = wifi_configs
            .iter()
            .find(|&x| x.identifier == *WIRELESS_OPEN_SAMPLE_FILENAME)
            .unwrap();
        assert_eq![
            wireless_open_config,
            &get_expected_wifi_config(WIRELESS_OPEN_SAMPLE_FILENAME)
        ];
    }

    #[test]
    fn test_find_and_parse_wired_configs() {
        let opts = GlobalOptions::default();
        let handler = get_sample_handler(&opts);

        let wired_configs: Vec<WiredNetctlConfig> = handler.get_all_typed_configs().unwrap();
        dbg![&wired_configs];

        let ethernet_sample_res = wired_configs
            .iter()
            .find(|&x| x.identifier == *ETHERNET_SAMPLE_FILENAME)
            .unwrap();
        assert_eq![
            ethernet_sample_res,
            &get_expected_wired_config(ETHERNET_SAMPLE_FILENAME)
        ];

        let wireless_encrypted_config = wired_configs
            .iter()
            .find(|&x| x.identifier == *WIRELESS_ENCRYPTED_SAMPLE_FILENAME);
        assert![wireless_encrypted_config.is_none()];

        let wireless_open_config = wired_configs
            .iter()
            .find(|&x| x.identifier == *WIRELESS_OPEN_SAMPLE_FILENAME);
        assert![wireless_open_config.is_none()];
    }

    #[test]
    fn test_get_matching_config() {
        let opts = GlobalOptions::default();
        let handler = get_sample_handler(&opts);

        let interface_name = "wlp3s1";
        let criteria = WifiNetctlConfigFinderCriteria::builder()
            .interface_name(interface_name)
            .build();

        let configs = handler
            .find_matching_configs::<WifiNetctlConfig>(&criteria)
            .unwrap();
        assert_eq![1, configs.len()];
        let config = configs.first().unwrap();
        assert_eq!["kingship_lobby", &config.identifier];
    }
}
