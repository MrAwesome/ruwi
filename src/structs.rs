use crate::errbox;
use std::error::Error;
use std::fmt::Debug;
use std::process::Output;
use strum_macros::{AsStaticStr, Display, EnumIter, EnumString};

pub(crate) type ErrBox = Box<dyn Error + Send + Sync>;

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
pub enum ScanType {
    IW,
    #[strum(serialize = "iwlist")]
    IWList,
    WpaCli,
}

impl Default for ScanType {
    fn default() -> Self {
        ScanType::IW
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum OutputType {
    None,
    PrintSelectedNetwork,
    PrintSelectedNetworkInfo,
    NetctlConfig,
    #[strum(serialize = "networkmanager_config")]
    NetworkManagerConfig,
}

impl Default for OutputType {
    fn default() -> Self {
        // TODO: come up with a better value
        OutputType::NetctlConfig
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
        SelectionMethod::Dmenu
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum ConnectionType {
    None,
    Netctl,
    NetworkManager,
}

impl Default for ConnectionType {
    fn default() -> Self {
        ConnectionType::Netctl
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    pub scan_type: ScanType,
    pub selection_method: SelectionMethod,
    pub interface: String,
    pub output_type: OutputType,
    pub connect_via: ConnectionType,
    pub debug: bool,
    pub given_essid: Option<String>,
    pub given_password: Option<String>,
}

impl Options {
    pub fn dbg<T: Debug>(&self, lawl: T) {
        dbg![lawl];
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            scan_type: ScanType::IW,
            interface: "some_fake_name".to_string(),
            selection_method: SelectionMethod::Fzf,
            output_type: OutputType::None,
            connect_via: ConnectionType::None,
            debug: true,
            given_essid: None,
            given_password: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanResult {
    pub scan_type: ScanType,
    pub scan_output: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    MissingWpaCliResultField,
    ZeroLengthIWChunk,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WirelessNetwork {
    pub essid: String,
    pub is_encrypted: bool,
    pub bssid: Option<String>,
    pub signal_strength: Option<i32>,
    pub channel_utilisation: Option<String>,
}

impl Default for WirelessNetwork {
    fn default() -> Self {
        WirelessNetwork {
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
    pub fn set_known(&mut self) {
        self.known = true;
    }
}

impl Default for AnnotatedWirelessNetwork {
    fn default() -> Self {
        let nw = WirelessNetwork::default();
        AnnotatedWirelessNetwork::from(nw)
    }
}

impl From<WirelessNetwork> for AnnotatedWirelessNetwork {
    fn from(nw: WirelessNetwork) -> Self {
        let essid = nw.essid;
        let is_encrypted = nw.is_encrypted;
        let bssid = nw.bssid;
        let signal_strength = nw.signal_strength;
        let channel_utilisation = nw.channel_utilisation;
        AnnotatedWirelessNetwork {
            essid,
            is_encrypted,
            bssid,
            signal_strength,
            channel_utilisation,
            known: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnnotatedNetworks {
    pub networks: Vec<AnnotatedWirelessNetwork>,
}

#[derive(Debug, Clone)]
pub struct SortedNetworks {
    pub networks: Vec<AnnotatedWirelessNetwork>,
}

impl From<AnnotatedNetworks> for SortedNetworks {
    fn from(nws: AnnotatedNetworks) -> Self {
        SortedNetworks {
            networks: nws.networks,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputResult {
    pub output_type: OutputType,
    pub output_output: Option<Output>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionResult {
    pub connection_type: ConnectionType,
    pub cmd_output: Option<String>,
    //ipv4_addr: Option<String>,
    //ipv6_addr: Option<String>,
}

#[derive(Debug)]
pub struct RuwiResult {
    pub output_result: OutputResult,
    pub connection_result: ConnectionResult,
}

pub(crate) fn nie<T: Debug>(prog: T) -> ErrBox {
    errbox!(format!("Not implemented: {:?}", prog))
}
