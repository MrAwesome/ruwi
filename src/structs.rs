use std::fmt::Debug;
use std::io;

pub fn nie<T: Debug>(prog: T) -> io::Error {
    println!();
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("Not implemented: {:?}", prog),
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScanType {
    IW,
    IWList,
    WpaCli,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputType {
    ListAllNetworks,
    PrintAllNetworksInfo,
    PrintSelectedNetwork,
    PrintSelectedNetworkInfo,
    NetctlConfig,
    NetworkManagerConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionMethod {
    Dmenu,
    Fzf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionType {
    Netctl,
    NetworkManager,
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

#[derive(Debug, Clone, Eq, PartialEq)]
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionResult {
    pub connection_type: ConnectionType,
    pub result: Result<String, String>,
    //ipv4_addr: Option<String>,
    //ipv6_addr: Option<String>,
}
