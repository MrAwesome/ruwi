use std::fmt::Debug;
use std::io;
use std::process::Output;
use strum_macros::{AsStaticStr, Display, EnumIter, EnumString};

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
        ScanType::WpaCli
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum OutputType {
    None,
    ListAllNetworks,
    PrintInfoForAllNetworks,
    PrintSelectedNetwork,
    PrintSelectedNetworkInfo,
    NetctlConfig,
    #[strum(serialize = "networkmanager_config")]
    NetworkManagerConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum SelectionMethod {
    Dmenu,
    Fzf,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ConnectionType {
    Netctl,
    NetworkManager,
    None,
}

#[derive(Debug, Clone)]
pub struct Options {
    pub scan_type: ScanType,
    pub selection_method: SelectionMethod,
    // TODO: make Option, and scan for interface if not given
    pub interface: String,
    pub output_types: Vec<OutputType>,
    pub connect_via: Option<ConnectionType>,
    pub debug: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanResult {
    pub scan_type: ScanType,
    pub output: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseResult {
    pub scan_type: ScanType,
    pub seen_networks: Vec<WirelessNetwork>,
    pub line_parse_errors: Vec<(String, IndividualParseError)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndividualParseError {
    SplitError,
    MissingWpaCliResultField,
    FailedToParseSignalLevel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WirelessNetwork {
    pub essid: String,
    pub wpa: bool,
    pub bssid: Option<String>,
    pub signal_strength: Option<i32>,
    pub channel_utilisation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputResult {
    pub output_type: OutputType,
    pub output_output: Option<Output>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionResult {
    pub connection_type: ConnectionType,
    pub cmd_output: Option<Output>,
    //ipv4_addr: Option<String>,
    //ipv6_addr: Option<String>,
}

#[derive(Debug)]
pub struct RuwiResult {
    pub output_results: Vec<io::Result<OutputResult>>,
    pub connection_result: io::Result<ConnectionResult>,
}

pub fn nie<T: Debug>(prog: T) -> io::Error {
    println!();
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("Not implemented: {:?}", prog),
    )
}
