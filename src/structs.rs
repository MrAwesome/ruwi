// For strum macros:
#![allow(clippy::default_trait_access)]
#![allow(clippy::used_underscore_binding)]

use std::fmt::Debug;

// NOTE: instead of strum, you can use arg_enum! from the clap crate, to cut down on compile times
use strum_macros::{AsStaticStr, Display, EnumIter, EnumString};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScanMethod {
    ByRunning,
    FromFile(String),
    FromStdin,
}

impl Default for ScanMethod {
    fn default() -> Self {
        Self::ByRunning
    }
}

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
pub enum ScanType {
    Wifi(WifiScanType),
}

impl Default for ScanType {
    fn default() -> Self {
        Self::Wifi(WifiScanType::default())
    }
}

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
pub enum WifiScanType {
    IW,
    WpaCli,
    RuwiJSON,
    Nmcli,
    //#[strum(serialize = "iwlist")]
    //IWList,
}

impl Default for WifiScanType {
    fn default() -> Self {
        Self::IW
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum SelectionMethod {
    Dmenu,
    Fzf,
}

impl Default for SelectionMethod {
    fn default() -> Self {
        Self::Fzf
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum SelectionOption {
    Refresh,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum WifiConnectionType {
    None,
    Netctl,
    Nmcli,
    Print,
    // PrintWithPassword
}

impl Default for WifiConnectionType {
    fn default() -> Self {
        Self::Netctl
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum AutoMode {
    Ask,
    KnownOrAsk,
    KnownOrFail,
    First,
}

impl Default for AutoMode {
    fn default() -> Self {
        Self::Ask
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SynchronousRescanType {
    ManuallyRequested,
    Automatic,
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotatedWirelessNetwork {
    pub essid: String,
    pub is_encrypted: bool,
    pub bssid: Option<String>,
    pub signal_strength: Option<i32>,
    pub channel_utilisation: Option<String>,
    pub known: bool,
}

impl AnnotatedWirelessNetwork {
    pub fn from_nw(nw: WirelessNetwork, is_known: bool) -> Self {
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
