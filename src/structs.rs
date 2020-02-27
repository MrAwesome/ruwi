// For typedbuilder:
#![allow(clippy::used_underscore_binding)]

use std::fmt::Debug;
use crate::enums::*;
use crate::options::interfaces::*;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScanResult {
    pub scan_type: ScanType,
    pub scan_output: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParseResult {
    pub scan_type: ScanType,
    pub seen_networks: Vec<WirelessNetwork>,
    pub line_parse_errors: Vec<(String, IndividualParseError)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndividualParseError {
    FailedToParseSignalLevel,
    FailedToUnescapeSSIDField,
    MissingIWCapabilityField,
    MissingIWSSIDField,
    MissingNmcliSeparator,
    MissingWpaCliResultField,
    ZeroLengthIWChunk,
}

// TODO: make private, provide interface?
#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub struct WirelessNetwork {
    pub essid: String,
    pub is_encrypted: bool,
    pub bssid: Option<String>,
    pub signal_strength: Option<i32>,
    pub channel_utilisation: Option<String>,
}

impl Default for WirelessNetwork {
    fn default() -> Self {
        Self {
            essid: "FAKE_ESSID_SHOULD_NOT_BE_SEEN".to_string(),
            is_encrypted: false,
            bssid: None,
            signal_strength: None,
            channel_utilisation: None,
        }
    }
}

impl RuwiNetwork for WirelessNetwork {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotatedWirelessNetwork {
    pub essid: String,
    pub is_encrypted: bool,
    pub bssid: Option<String>,
    pub signal_strength: Option<i32>,
    pub channel_utilisation: Option<String>,
    pub known: bool,
}

impl Annotated<WirelessNetwork> for AnnotatedWirelessNetwork {
    fn from_nw(nw: WirelessNetwork, is_known: bool) -> Self {
        let essid = nw.essid;
        let is_encrypted = nw.is_encrypted;
        let bssid = nw.bssid;
        let signal_strength = nw.signal_strength;
        let channel_utilisation = nw.channel_utilisation;
        Self {
            essid,
            is_encrypted,
            bssid,
            signal_strength,
            channel_utilisation,
            known: is_known,
        }
    }
}

impl AnnotatedWirelessNetwork {
    pub fn from_essid(essid: String, is_known: bool, is_encrypted: bool) -> Self {
        Self {
            essid,
            is_encrypted,
            known: is_known,
            ..Self::default()
        }
    }
}

impl Default for AnnotatedWirelessNetwork {
    fn default() -> Self {
        let nw = WirelessNetwork::default();
        Self::from_nw(nw, false)
    }
}

impl Identifiable for WirelessNetwork {
    fn get_identifier(&self) -> &str {
        self.essid.as_ref()
    }
}

impl Identifiable for AnnotatedWirelessNetwork {
    fn get_identifier(&self) -> &str {
        self.essid.as_ref()
    }
}

impl Known for AnnotatedWirelessNetwork {
    fn is_known(&self) -> bool {
        self.known
    }
}

impl RuwiNetwork for AnnotatedWirelessNetwork {}
impl AnnotatedRuwiNetwork for AnnotatedWirelessNetwork {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigResult {
    pub connection_type: WifiConnectionType,
    pub config_data: ConfigData,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConfigData {
    pub config_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionResult {
    pub connection_type: WifiConnectionType,
    //pub cmd_output: Option<String>,
    //ipv4_addr: Option<String>,
    //ipv6_addr: Option<String>,
}
