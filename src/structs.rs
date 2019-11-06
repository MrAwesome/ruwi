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

#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
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

// TODO: determine more intelligently?
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
    MissingIWSSIDField,
    ZeroLengthIWChunk,
    MalformedIWBSSLine,
    FailedToUnescapeSSIDField,
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
    pub output_result: io::Result<OutputResult>,
    pub connection_result: io::Result<ConnectionResult>,
}

pub(crate) fn nie<T: Debug>(prog: T) -> io::Error {
    println!();
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("Not implemented: {:?}", prog),
    )
}
