use super::config_finder::*;
use super::utils::check_for_field;
use crate::common::*;
use crate::string_container;

use strum_macros::AsStaticStr;
use strum_macros::{Display, EnumString};
use typed_builder::TypedBuilder;

use std::convert::TryFrom;
use std::str::FromStr;

string_container! {NetctlIdentifier, NetctlRawConfigContents}

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
    type Error = RuwiError;

    fn try_from(raw: &NetctlRawConfig) -> Result<Self, RuwiError> {
        // TODO:
        // [] handle errors gracefully, don't just kill ruwi because a single config doesn't parse
        let identifier = raw.identifier.clone();
        let connection_type_text = raw.get_connection_type().ok_or(rerr!(
            RuwiErrorKind::MissingFieldInNetctlConfig,
            format!(
                "No connection type found for config {}!",
                raw.identifier.as_ref()
            ),
        ))?;
        let connection_type =
            NetctlConnectionType::from_str(&connection_type_text).map_err(|_e| {
                rerr!(
                    RuwiErrorKind::MissingFieldInNetctlConfig,
                    format!(
                        "Failed to parse connection type \"{}\" found in config {}!",
                        &connection_type_text,
                        raw.identifier.as_ref()
                    ),
                )
            })?;
        let interface_name = raw.get_interface().ok_or(rerr!(
            RuwiErrorKind::MissingFieldInNetctlConfig,
            format!(
                "No interface found for config {}!",
                raw.identifier.as_ref()
            ),
        ))?;

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
pub(super) struct WiredNetctlConfig {
    pub(super) identifier: NetctlIdentifier,
    pub(super) interface_name: String,
}

impl<'a> NetctlConfig<'a> for WiredNetctlConfig {
    type Checker = WiredNetctlConfigFinderCriteria<'a>;

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
