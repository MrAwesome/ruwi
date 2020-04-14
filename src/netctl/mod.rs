pub(crate) mod config_finder;
use config_finder::*;
pub(crate) mod config_reader;
pub(crate) mod config_writer;
pub(crate) mod utils;

use config_reader::read_all_netctl_config_files;

use crate::common::*;
use crate::interface_management::ip_interfaces::*;
use crate::string_container;
use utils::*;

use std::convert::TryFrom;
use strum_macros::Display;
use typed_builder::TypedBuilder;

const DEFAULT_NETCTL_CFG_DIR: &str = "/etc/netctl/";

string_container! {NetctlIdentifier, NetctlRawConfigContents}

impl From<&AnnotatedWirelessNetwork> for NetctlIdentifier {
    fn from(nw: &AnnotatedWirelessNetwork) -> Self {
        let ident = match nw.get_service_identifier() {
            Some(NetworkServiceIdentifier::Netctl(ident)) => ident.clone(),
            _ => nw.get_public_name().replace(" ", "_"),
        };
        Self::new(ident)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
enum NetctlConnectionType {
    #[strum(serialize = "wireless")]
    Wifi,
    #[strum(serialize = "ethernet")]
    Wired,
}

trait NetctlConfig: Display + TryFrom<NetctlRawParsedFields> {
    type Checker: NetctlConfigFinderCriteria;

    fn get_identifier(&self) -> &NetctlIdentifier;
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
struct NetctlRawConfig<'a> {
    identifier: NetctlIdentifier,
    contents: NetctlRawConfigContents,
    location: &'a str,
}

impl<'a> NetctlRawConfig<'a> {
    fn new(
        identifier: NetctlIdentifier,
        contents: NetctlRawConfigContents,
        location: &'a str,
    ) -> Self {
        Self {
            identifier,
            contents,
            location,
        }
    }

    fn get_contents(&self) -> &NetctlRawConfigContents {
        &self.contents
    }

    fn get_identifier(&self) -> &NetctlIdentifier {
        &self.identifier
    }

    fn get_location(&self) -> &str {
        self.location
    }
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
struct NetctlRawParsedFields {
    identifier: NetctlIdentifier,
    connection_type: NetctlConnectionType,
    interface_name: String,
    essid: Option<String>,
    encryption_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
struct WifiNetctlConfig {
    identifier: NetctlIdentifier,
    essid: String,
    interface_name: String,
    encryption_key: Option<String>,
}

impl NetctlConfig for WifiNetctlConfig {
    type Checker = WifiNetctlConfigFinderCriteria;

    fn get_identifier(&self) -> &NetctlIdentifier {
        &self.identifier
    }
}

impl TryFrom<NetctlRawParsedFields> for WifiNetctlConfig {
    type Error = RuwiError;

    fn try_from(f: NetctlRawParsedFields) -> Result<Self, Self::Error> {
        let identifier = f.identifier;
        let interface_name = f.interface_name;
        let essid = check_for_field(&f.essid, &identifier, "ESSID")?;
        let encryption_key = f.encryption_key;
        Ok(Self::builder()
            .identifier(identifier)
            .interface_name(interface_name)
            .essid(essid)
            .encryption_key(encryption_key)
            .build())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
struct WiredNetctlConfig {
    identifier: NetctlIdentifier,
    interface_name: String,
}

impl NetctlConfig for WiredNetctlConfig {
    type Checker = WiredNetctlConfigFinderCriteria;

    fn get_identifier(&self) -> &NetctlIdentifier {
        &self.identifier
    }
}

impl TryFrom<NetctlRawParsedFields> for WiredNetctlConfig {
    type Error = RuwiError;

    fn try_from(f: NetctlRawParsedFields) -> Result<Self, RuwiError> {
        let identifier = f.identifier;
        let interface_name = f.interface_name;
        Ok(Self::builder()
            .identifier(identifier)
            .interface_name(interface_name)
            .build())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub(crate) struct NetctlConfigHandler<'a, O: Global> {
    opts: &'a O,
    #[builder(default = DEFAULT_NETCTL_CFG_DIR.to_string())]
    netctl_cfg_dir: String,
}

impl<'a, O: Global> NetctlConfigHandler<'a, O> {
    pub(crate) fn new(opts: &'a O) -> Self {
        NetctlConfigHandler::builder().opts(opts).build()
    }

    fn get_netctl_cfg_dir(&self) -> &str {
        &self.netctl_cfg_dir
    }

    fn get_all_configs_text(&'a self) -> Result<Vec<NetctlRawConfig<'a>>, RuwiError> {
        read_all_netctl_config_files(self.netctl_cfg_dir.as_ref())
    }

    fn get_all_typed_configs<C>(&self) -> Result<Vec<C>, RuwiError>
    where
        C: NetctlConfig,
    {
        let configs_text = self.get_all_configs_text()?;
        let raw_parsed_configs = configs_text
            .iter()
            .filter_map(|text| NetctlRawParsedFields::try_from(text).ok())
            .filter_map(|config| C::try_from(config).ok())
            .collect::<Vec<C>>();

        Ok(raw_parsed_configs)
    }

    fn find_matching_configs<C, K>(&self, criteria: &C::Checker) -> Result<Vec<C>, RuwiError>
    where
        C: NetctlConfig,
    {
        let all_typed_configs = self.get_all_typed_configs()?;
        let selected_typed_configs = criteria.select(&all_typed_configs);
        todo!("use criteria");
        //        Ok(configs_text
        //            .iter()
        //            .filter_map(|text| NetctlRawParsedFields::try_from(text).ok())
        //            .collect())
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

// TODO: function to get all netctl config contents
// TODO: function to parse all seen netctl contents into readable structs
// TODO: function to filter those structs based on Criteria
// TODO: function to convert those structs into expected Config objects
