use super::config_finder::*;
use super::utils::*;
use super::NetctlIdentifier;

use crate::string_container;
use crate::prelude::*;

use strum_macros::AsStaticStr;
use strum_macros::{Display, EnumString};
use typed_builder::TypedBuilder;

use std::convert::TryFrom;
use std::str::FromStr;

string_container! {NetctlRawConfigContents}

pub(super) trait NetctlConfig<'a>: fmt::Display + TryFrom<NetctlRawParsedFields> {
    type Checker: NetctlConfigFinderCriteria<'a>;

    fn get_identifier(&self) -> &NetctlIdentifier;
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub(super) struct NetctlRawConfig<'a> {
    // TODO: get rid of pub(super) here
    pub(super) identifier: NetctlIdentifier,
    pub(super) contents: NetctlRawConfigContents,
    // TODO: is location necessary here? can you just pass it around separately if needed?
    pub(super) location: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub(super) struct NetctlRawParsedFields {
    pub(super) identifier: NetctlIdentifier,
    pub(super) connection_type: NetctlConnectionType,
    pub(super) interface_name: String,
    pub(super) essid: Option<String>,
    pub(super) encryption_key: Option<String>,
}

// TODO: unit test in own file
impl<'a> TryFrom<&NetctlRawConfig<'a>> for NetctlRawParsedFields {
    type Error = NetctlParseError;

    fn try_from(raw: &NetctlRawConfig) -> Result<Self, Self::Error> {
        // TODO:
        // [] handle errors gracefully, don't just kill ruwi because a single config doesn't parse
        let identifier = raw.identifier.clone();
        // TODO TODO: decide if netctl parse errors should own some error text to present to the
        // user later
        let connection_type_text =
            raw.get_connection_type()
                .ok_or(NetctlParseError::MissingFieldInNetctlConfig(format!(
                    "No connection type found for config {}!",
                    raw.identifier.as_ref()
                )))?;
        let connection_type =
            NetctlConnectionType::from_str(&connection_type_text).map_err(|_e| {
                NetctlParseError::MissingFieldInNetctlConfig(format!(
                    "Failed to parse connection type \"{}\" found in config {}!",
                    &connection_type_text,
                    raw.identifier.as_ref()
                ))
            })?;
        let interface_name =
            raw.get_interface()
                .ok_or(NetctlParseError::MissingFieldInNetctlConfig(format!(
                    "No interface found for config {}!",
                    raw.identifier.as_ref()
                )))?;

        let essid = raw.get_essid();
        let encryption_key = raw.get_encryption_key();

        Ok(Self::builder()
            .identifier(identifier)
            .connection_type(connection_type)
            .interface_name(interface_name)
            .essid(essid)
            .encryption_key(encryption_key)
            .build())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Display, EnumString, AsStaticStr)]
pub(super) enum NetctlConnectionType {
    #[strum(serialize = "wireless")]
    Wifi,
    #[strum(serialize = "ethernet")]
    Wired,
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub(super) struct WifiNetctlConfig {
    pub(super) identifier: NetctlIdentifier,
    pub(super) essid: String,
    pub(super) interface_name: String,
    pub(super) encryption_key: Option<String>,
}

impl<'a> NetctlConfig<'a> for WifiNetctlConfig {
    type Checker = WifiNetctlConfigFinderCriteria<'a>;

    fn get_identifier(&self) -> &NetctlIdentifier {
        &self.identifier
    }
}

impl WifiNetctlConfig {
    pub(super) fn get_essid(&self) -> &str {
        self.essid.as_ref()
    }
}

impl TryFrom<NetctlRawParsedFields> for WifiNetctlConfig {
    type Error = NetctlParseError;

    fn try_from(f: NetctlRawParsedFields) -> Result<Self, Self::Error> {
        check_connection_type(NetctlConnectionType::Wifi, f.connection_type)?;
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
pub(crate) struct WiredNetctlConfig {
    pub(super) identifier: NetctlIdentifier,
    pub(super) interface_name: String,
}

impl From<&WiredNetctlConfig> for AnnotatedWiredNetwork {
    fn from(config: &WiredNetctlConfig) -> Self {
        AnnotatedWiredNetwork::builder()
            .interface(config.interface_name.clone())
            .service_identifier(NetworkServiceIdentifier::Netctl(config.identifier.as_ref().to_string()))
            .build()
            
    }
}

impl<'a> NetctlConfig<'a> for WiredNetctlConfig {
    type Checker = WiredNetctlConfigFinderCriteria<'a>;

    fn get_identifier(&self) -> &NetctlIdentifier {
        &self.identifier
    }
}

impl TryFrom<NetctlRawParsedFields> for WiredNetctlConfig {
    type Error = NetctlParseError;

    fn try_from(f: NetctlRawParsedFields) -> Result<Self, Self::Error> {
        check_connection_type(NetctlConnectionType::Wired, f.connection_type)?;
        let identifier = f.identifier;
        let interface_name = f.interface_name;

        Ok(Self::builder()
            .identifier(identifier)
            .interface_name(interface_name)
            .build())
    }
}

pub(super) enum NetctlParseError {
    IncorrectConnectionType,
    MissingFieldInNetctlConfig(String),
}
