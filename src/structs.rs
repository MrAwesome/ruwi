use crate::rerr;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use strum_macros::{AsStaticStr, Display, EnumIter, EnumString};

pub static PROG_NAME: &str = "ruwi";

// TODO: set to pub(crate) temporarily to find unused values
#[derive(Debug, PartialEq, Eq)]
pub enum RuwiErrorKind {
    InvalidScanTypeAndMethod,
    FailedToListKnownNetworksWithNetworkManager,
    FailedToBringInterfaceDown,
    FailedToBringInterfaceUp,
    FailedToConnectViaNetctl,
    FailedToConnectViaNetworkManager,
    FailedToConnectViaWPACli,
    FailedToParseSelectedLine,
    FailedToReadScanResultsFromStdin,
    FailedToReadScanResultsFromFile,
    FailedToRunCommand,
    FailedToRunIWDev,
    FailedToRunIWScanAbort,
    FailedToRunIWScanDump,
    FailedToRunIWScanTrigger,
    FailedToScanWithWPACli,
    FailedToSpawnThread,
    FailedToWriteNetctlConfig,
    IWSynchronousScanFailed,
    IWSynchronousScanRanOutOfRetries,
    KnownNetworksFetchError,
    MalformedIWOutput,
    NoInterfacesFoundWithIW,
    NoKnownNetworksFound,
    NoNetworksFoundMatchingSelectionResult,
    NoNetworksSeenWithIWScanDump,
    NoNetworksSeenWithWPACliScanResults,
    NotImplementedError,
    PromptCommandFailed,
    PromptCommandSpawnFailed,
    RefreshRequested,
    RetryWithSynchronousScan,
    SingleLinePromptFailed,
    TestCmdLineOptParserSafeFailed,
    TestDeliberatelyFailedToFindNetworks,
    TestNoNetworksFoundWhenLookingForFirst,
    TestNoRefreshOptionFound,
    TestNoNetworksFoundWhenLookingForLast,
    TestUsedAutoNoAskWhenNotExpected,
    TestUsedAutoWhenNotExpected,
    TestUsedManualWhenNotExpected,
    WPACliHeaderMalformedOrMissing,
}

#[derive(Debug)]
pub struct RuwiError {
    pub kind: RuwiErrorKind,
    pub desc: String,
}

impl Error for RuwiError {
    fn description(&self) -> &str {
        self.desc.as_ref()
    }
}

impl fmt::Display for RuwiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.description(), f)
    }
}

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
    IW,
    WpaCli,
    RuwiJSON,
    //#[strum(serialize = "iwlist")]
    //IWList,
}

impl Default for ScanType {
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
pub enum ConnectionType {
    None,
    Netctl,
    #[strum(serialize = "networkmanager")]
    NetworkManager,
    Print,
    // PrintWithPassword
}

impl Default for ConnectionType {
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
pub enum SynchronousRetryType {
    ManuallyRequested,
    Automatic,
}

#[derive(Debug, Clone)]
pub struct Options {
    pub scan_type: ScanType,
    pub scan_method: ScanMethod,
    pub selection_method: SelectionMethod,
    pub interface: String,
    pub connect_via: ConnectionType,
    pub debug: bool,
    pub given_essid: Option<String>,
    pub given_encryption_key: Option<String>,
    pub auto_mode: AutoMode,
    pub force_synchronous_scan: bool,
    pub synchronous_retry: Option<SynchronousRetryType>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            scan_type: Default::default(),
            scan_method: Default::default(),
            // TODO: prevent this from ever being seen? How?
            interface: "some_fake_name".to_string(),
            selection_method: Default::default(),
            connect_via: Default::default(),
            debug: false,
            given_essid: None,
            given_encryption_key: None,
            auto_mode: Default::default(),
            force_synchronous_scan: false,
            synchronous_retry: None,
        }
    }
}

impl Options {
    pub fn with_synchronous_retry(&self, t: SynchronousRetryType) -> Self {
        Options {
            synchronous_retry: Some(t.clone()),
            ..self.clone()
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
        AnnotatedWirelessNetwork {
            essid,
            is_encrypted,
            bssid,
            signal_strength,
            channel_utilisation,
            known: is_known,
        }
    }

    pub fn from_essid(essid: String, is_known: bool, is_encrypted: bool) -> Self {
        AnnotatedWirelessNetwork {
            essid,
            is_encrypted,
            known: is_known,
            ..Default::default()
        }
    }
}

impl Default for AnnotatedWirelessNetwork {
    fn default() -> Self {
        let nw = WirelessNetwork::default();
        AnnotatedWirelessNetwork::from_nw(nw, false)
    }
}

#[derive(Debug, Clone)]
pub struct AnnotatedNetworks {
    pub networks: Vec<AnnotatedWirelessNetwork>,
}

#[derive(Debug, Clone)]
pub struct SortedUniqueNetworks {
    pub networks: Vec<AnnotatedWirelessNetwork>,
}

impl From<AnnotatedNetworks> for SortedUniqueNetworks {
    fn from(nws: AnnotatedNetworks) -> Self {
        SortedUniqueNetworks {
            networks: nws.networks,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigResult {
    pub connection_type: ConnectionType,
    pub config_data: ConfigData,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConfigData {
    pub config_path: Option<String>,
    //command_run: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionResult {
    pub connection_type: ConnectionType,
    //pub cmd_output: Option<String>,
    //ipv4_addr: Option<String>,
    //ipv6_addr: Option<String>,
}

pub(crate) fn nie<T: Debug>(prog: T) -> RuwiError {
    rerr!(
        RuwiErrorKind::NotImplementedError,
        format!("Functionality not implemented: {:?}", prog)
    )
}
