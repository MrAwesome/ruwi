use crate::common::*;
use crate::string_container;
use super::config_finder::*;
use super::utils::check_for_field;

use std::convert::TryFrom;
use strum_macros::Display;
use typed_builder::TypedBuilder;

string_container! {NetctlIdentifier, NetctlRawConfigContents}

pub(super) trait NetctlConfig: fmt::Display + TryFrom<NetctlRawParsedFields> {
    type Checker: NetctlConfigFinderCriteria;

    fn get_identifier(&self) -> &NetctlIdentifier;
}


#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub(super) struct NetctlRawConfig<'a> {
    pub(super) identifier: NetctlIdentifier,
    pub(super) contents: NetctlRawConfigContents,
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

impl<'a> TryFrom<&NetctlRawConfig<'a>> for NetctlRawParsedFields {
    type Error = RuwiError;

    fn try_from(f: &NetctlRawConfig) -> Result<Self, RuwiError> {
        // TODO: here, or in a helper function, grab the fields you need from a netctl config blob
        unimplemented!()
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Display)]
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
pub(super) struct WiredNetctlConfig {
    pub(super) identifier: NetctlIdentifier,
    pub(super) interface_name: String,
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

