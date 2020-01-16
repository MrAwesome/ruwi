use crate::rerr;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use strum_macros::{AsStaticStr, Display, EnumIter, EnumString};

pub static PROG_NAME: &str = "ruwi";

// TODO: Strum
#[derive(Debug, Clone)]
pub enum RuwiCommand {
    Wifi(RuwiWifiCommand),
}

impl Default for RuwiCommand {
    fn default() -> Self {
        Self::Wifi(RuwiWifiCommand::default())
    }
}

#[derive(Debug, Clone)]
pub enum RuwiWifiCommand {
    Connect
    // Select
    // JSON
}

impl Default for RuwiWifiCommand {
    fn default() -> Self {
        Self::Connect
    }
}


#[derive(Debug, Clone)]
pub struct Options {
    pub scan_type: ScanType,
    pub scan_method: ScanMethod,
    pub selection_method: SelectionMethod,
    pub interface: String,
    pub ignore_known: bool,
    pub connect_via: ConnectionType,
    pub debug: bool,
    pub given_essid: Option<String>,
    pub given_encryption_key: Option<String>,
    pub auto_mode: AutoMode,
    pub force_synchronous_scan: bool,
    pub force_ask_password: bool,
    pub synchronous_retry: Option<SynchronousRescanType>,
    pub dry_run: bool,
    pub use_state_machine: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            scan_type: ScanType::default(),
            scan_method: ScanMethod::default(),
            interface: "wlan0".to_string(),
            ignore_known: false,
            selection_method: SelectionMethod::default(),
            connect_via: ConnectionType::default(),
            debug: false,
            given_essid: None,
            given_encryption_key: None,
            auto_mode: AutoMode::default(),
            force_synchronous_scan: false,
            force_ask_password: false,
            synchronous_retry: None,
            #[cfg(not(test))]
            dry_run: false,
            #[cfg(test)]
            dry_run: true,
            use_state_machine: false,
        }
    }
}

impl Options {
    pub fn from_scan_type(scan_type: &ScanType) -> Self {
        Self {
            scan_type: scan_type.clone(),
            ..Self::default()
        }
    }
    pub fn with_synchronous_retry(&self, t: SynchronousRescanType) -> Self {
        Self {
            synchronous_retry: Some(t),
            ..self.clone()
        }
    }
}

// TODO: set to pub(crate) temporarily to find unused values
#[derive(Debug, PartialEq, Eq)]
pub enum RuwiErrorKind {
    CommandNotInstalled,
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
    FailedToRunNmcliScan,
    FailedToRunNmcliScanSynchronous,
    FailedToScanWithWPACli,
    FailedToSpawnThread,
    FailedToStartNetctl,
    FailedToStartNetworkManager,
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
    StepRunnerLoopPreventionCapExceeded,
    TestCmdLineOptParserSafeFailed,
    TestDeliberatelyFailedToFindNetworks,
    TestNoNetworksFoundWhenLookingForFirst,
    TestNoRefreshOptionFound,
    TestNoNetworksFoundWhenLookingForLast,
    TestShouldNeverBeSeen,
    TestUsedAutoNoAskWhenNotExpected,
    TestUsedAutoWhenNotExpected,
    TestUsedManualWhenNotExpected,
    UsedTerminalStep,
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
    Nmcli,
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
    // TODO: should actually be nmcli
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

#[derive(Debug, PartialEq, Eq)]
pub struct KnownNetworkNames(pub HashSet<String>);

impl Default for KnownNetworkNames {
    fn default() -> Self {
        Self(HashSet::new())
    }
}

impl Deref for KnownNetworkNames {
    type Target = HashSet<String>;
    fn deref(&self) -> &HashSet<String> {
        &self.0
    }
}

impl DerefMut for KnownNetworkNames {
    fn deref_mut(&mut self) -> &mut HashSet<String> {
        &mut self.0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AnnotatedNetworks {
    pub networks: Vec<AnnotatedWirelessNetwork>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SortedUniqueNetworks {
    pub networks: Vec<AnnotatedWirelessNetwork>,
}

impl From<AnnotatedNetworks> for SortedUniqueNetworks {
    fn from(nws: AnnotatedNetworks) -> Self {
        Self {
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
