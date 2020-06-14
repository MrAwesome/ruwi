mod config_finder;
mod config_parser;
mod config_reader;
mod config_writer;
mod structs;
mod utils;

use config_finder::{NetctlConfigFinderCriteria, WiredNetctlConfigFinderCriteria};
#[cfg(not(test))]
use config_reader::reader_implementation::read_all_netctl_config_files;
#[cfg(test)]
use structs::NetctlRawConfigContents;
use structs::{
    NetctlConfig, NetctlRawConfig, NetctlRawParsedFields, WifiNetctlConfig, WiredNetctlConfig,
};

use crate::interface_management::ip_interfaces::{WifiIPInterface, WiredIPInterface};
use crate::prelude::*;
use std::convert::TryFrom;
use typed_builder::TypedBuilder;

use super::NetctlIdentifier;
use super::DEFAULT_NETCTL_CFG_DIR;

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub(crate) struct NetctlConfigHandler<'a, O: Global> {
    opts: &'a O,
    #[builder(default = DEFAULT_NETCTL_CFG_DIR.to_string())]
    netctl_cfg_dir: String,
    #[cfg(test)]
    #[builder(default)]
    given_configs: Vec<(NetctlIdentifier, NetctlRawConfigContents)>,
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
        Ok(self
            .given_configs
            .iter()
            .map(|(identifier, contents)| {
                NetctlRawConfig::builder()
                    .identifier(identifier.clone())
                    .contents(contents.clone())
                    .location(self.get_netctl_cfg_dir())
                    .build()
            })
            .collect())
    }

    #[cfg(not(test))]
    fn get_all_configs_text(&'a self) -> Result<Vec<NetctlRawConfig<'a>>, RuwiError> {
        read_all_netctl_config_files(self.get_netctl_cfg_dir())
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

    pub(crate) fn get_wired_configs(
        &self,
        ifname: &str,
    ) -> Result<Vec<WiredNetctlConfig>, RuwiError> {
        // TODO: allow for specifying a particular netctl profile for wired netctl connect
        let criteria = WiredNetctlConfigFinderCriteria::builder()
            .interface_name(ifname)
            .build();
        self.find_matching_configs::<WiredNetctlConfig>(&criteria)
    }

    pub(crate) fn get_wifi_essids_and_identifiers(
        &self,
    ) -> Result<Vec<(String, NetctlIdentifier)>, RuwiError> {
        Ok(self
            .get_all_typed_configs::<WifiNetctlConfig>()?
            .iter()
            .map(|config| {
                (
                    config.get_essid().to_string(),
                    config.get_identifier().clone(),
                )
            })
            .collect())
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

    pub(crate) fn write_wifi_config(
        &self,
        interface: &WifiIPInterface,
        network: &AnnotatedWirelessNetwork,
        encryption_key: &Option<String>,
    ) -> Result<NetctlIdentifier, RuwiError> {
        let config = WifiNetctlConfig::new(interface, network, encryption_key);

        self.write_config_to_file(&config)?;
        Ok(config.get_identifier().clone())
    }

    pub(crate) fn write_wired_config(
        &self,
        interface: &WiredIPInterface,
        network: &AnnotatedWiredNetwork,
    ) -> Result<NetctlIdentifier, RuwiError> {
        let config = WiredNetctlConfig::new(interface, network);

        self.write_config_to_file(&config)?;
        Ok(config.get_identifier().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::config_finder::WifiNetctlConfigFinderCriteria;
    use super::structs::NetctlConnectionType;
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

    fn get_sample_configs() -> Vec<(NetctlIdentifier, NetctlRawConfigContents)> {
        let locations = get_sample_filenames();
        locations
            .into_iter()
            .map(|filename| {
                (
                    NetctlIdentifier::from(filename),
                    NetctlRawConfigContents::from(get_sample_contents(filename)),
                )
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

    fn get_sample_handler<O: Global>(opts: &O) -> NetctlConfigHandler<'_, O> {
        let location = "samples/";
        let sample_configs = get_sample_configs();
        NetctlConfigHandler::builder()
            .opts(opts)
            .netctl_cfg_dir(location)
            .given_configs(sample_configs)
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
