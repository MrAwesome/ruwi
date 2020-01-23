use crate::rerr;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use strum_macros::{AsStaticStr, Display, EnumIter, EnumString, AsRefStr};
use typed_builder::TypedBuilder;

// NOTE: instead of strum, you can use arg_enum! from the clap crate, to cut down on compile times

pub static PROG_NAME: &str = "ruwi";

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, EnumString, EnumIter, Display, AsStaticStr, AsRefStr)]
pub enum RuwiCommand {
    Wifi(RuwiWifiCommand),
    Wired(RuwiWiredCommand),
    Bluetooth(RuwiBluetoothCommand),
}

impl RuwiCommand {
    // TODO: Obviously, this should return some trait?
    #[cfg(test)]
    pub fn get_options(&self) -> &WifiConnectOptions {
        if let Self::Wifi(RuwiWifiCommand::Connect(opts)) = self {
            opts
        } else {
            todo!("Get rid of this");
        }
    }
}

impl Default for RuwiCommand {
    fn default() -> Self {
        Self::Wifi(RuwiWifiCommand::default())
    }
}

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, EnumString, EnumIter, Display, AsStaticStr, AsRefStr)]
pub enum RuwiWifiCommand {
    Connect(WifiConnectOptions)
    // Select
    // JSON
}

impl Default for RuwiWifiCommand {
    fn default() -> Self {
        Self::Connect(WifiConnectOptions::default())
    }
}

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr, AsRefStr)]
pub enum RuwiWiredCommand {
    Connect
}

impl Default for RuwiWiredCommand {
    fn default() -> Self {
        Self::Connect
    }
}

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr, AsRefStr)]
pub enum RuwiBluetoothCommand {
    Pair
}

impl Default for RuwiBluetoothCommand {
    fn default() -> Self {
        Self::Pair
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct GlobalOptions {
    debug: bool,
    dry_run: bool,
    selection_method: SelectionMethod,
}

// Is this useful at all?
pub trait Global {
    fn d(&self) -> bool;
    fn get_debug(&self) -> bool;
    fn get_dry_run(&self) -> bool;
    fn get_selection_method(&self) -> &SelectionMethod;
}

impl Global for GlobalOptions {
    fn d(&self) -> bool {
        self.get_debug()
    }
    fn get_debug(&self) -> bool {
        self.debug
    }
    fn get_dry_run(&self) -> bool {
        self.dry_run
    }
    fn get_selection_method(&self) -> &SelectionMethod {
        &self.selection_method
    }
}

impl Default for GlobalOptions {
    fn default() -> Self {
        Self {
            debug: false,
            selection_method: SelectionMethod::default(),
            #[cfg(not(test))]
            dry_run: false,
            #[cfg(test)]
            dry_run: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BluetoothCommandOptions {
}

// TODO: use TypedBuilder for this, make globals and other fields private
#[derive(Debug, Clone, TypedBuilder)]
pub struct WifiOptions {
    // TODO: REMOVE GLOBALS FROM HERE?
    globals: GlobalOptions,
    #[builder(default)]
    scan_type: ScanType,
    #[builder(default)]
    scan_method: ScanMethod,
    #[builder(default="wlan0".to_string())]
    interface: String,
    #[builder(default=false)]
    ignore_known: bool,
    #[builder(default=None)]
    synchronous_retry: Option<SynchronousRescanType>,
    #[builder(default=false)]
    force_synchronous_scan: bool,
}

impl Default for WifiOptions {
    fn default() -> Self {
        Self {
            globals: GlobalOptions::default(),
            scan_type: ScanType::default(),
            scan_method: ScanMethod::default(),
            interface: "wlan0".to_string(),
            ignore_known: false,
            force_synchronous_scan: false,
            synchronous_retry: None,
        }
    }
}

impl WifiOptions {
    #[cfg(test)]
    pub fn from_scan_type(scan_type: ScanType) -> Self {
        Self {
            scan_type,
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

pub trait Wifi {
    fn get_scan_type(&self) -> &ScanType;
    fn get_scan_method(&self) -> &ScanMethod;
    fn get_interface(&self) -> &str;
    fn get_ignore_known(&self) -> bool;
    fn get_force_synchronous_scan(&self) -> bool;
    fn get_synchronous_retry(&self) -> &Option<SynchronousRescanType>;
}

impl Wifi for WifiOptions {
    fn get_scan_type(&self) -> &ScanType {
        &self.scan_type
    }
    fn get_scan_method(&self) -> &ScanMethod {
        &self.scan_method
    }
    fn get_interface(&self) -> &str {
        &self.interface
    }
    fn get_ignore_known(&self) -> bool {
        self.ignore_known
    }
    fn get_force_synchronous_scan(&self) -> bool {
        self.force_synchronous_scan
    }
    fn get_synchronous_retry(&self) -> &Option<SynchronousRescanType> {
        &self.synchronous_retry
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct WifiConnectOptions {
    #[builder(default)]
    globals: GlobalOptions,
    #[builder(default)]
    wifi: WifiOptions,
    #[builder(default)]
    auto_mode: AutoMode,
    #[builder(default)]
    connect_via: WifiConnectionType,
    #[builder(default=None)]
    given_essid: Option<String>,
    #[builder(default=false)]
    force_ask_password: bool,
    #[builder(default=None)]
    given_encryption_key: Option<String>,
}

impl Default for WifiConnectOptions {
    fn default() -> Self {
        Self {
            globals: GlobalOptions::default(),
            wifi: WifiOptions::default(),
            connect_via: WifiConnectionType::default(),
            given_essid: None,
            given_encryption_key: None,
            auto_mode: AutoMode::default(),
            force_ask_password: false,
        }
    }
}

pub trait WifiConnect {
    fn get_auto_mode(&self) -> &AutoMode;
    fn get_force_ask_password(&self) -> bool;
    fn get_given_essid(&self) -> &Option<String>;
    fn get_given_encryption_key(&self) -> &Option<String>;
    fn get_connect_via(&self) -> &WifiConnectionType;
}

impl Global for WifiConnectOptions {
    fn d(&self) -> bool {
        self.get_debug()
    }
    fn get_debug(&self) -> bool {
        self.globals.get_debug()
    }
    fn get_dry_run(&self) -> bool {
        self.globals.get_dry_run()
    }
    fn get_selection_method(&self) -> &SelectionMethod {
        self.globals.get_selection_method()
    }
}

impl Wifi for WifiConnectOptions {
    fn get_scan_type(&self) -> &ScanType {
        self.wifi.get_scan_type()
    }
    fn get_scan_method(&self) -> &ScanMethod {
        self.wifi.get_scan_method()
    }
    fn get_interface(&self) -> &str {
        self.wifi.get_interface()
    }
    fn get_ignore_known(&self) -> bool {
        self.wifi.get_ignore_known()
    }
    fn get_force_synchronous_scan(&self) -> bool {
        self.wifi.get_force_synchronous_scan()
    }
    fn get_synchronous_retry(&self) -> &Option<SynchronousRescanType> {
        self.wifi.get_synchronous_retry()
    }
}

impl WifiConnect for WifiConnectOptions {
    fn get_auto_mode(&self) -> &AutoMode {
        &self.auto_mode
    }
    fn get_force_ask_password(&self) -> bool {
        self.force_ask_password
    }
    fn get_given_essid(&self) -> &Option<String> {
        &self.given_essid
    }
    fn get_given_encryption_key(&self) -> &Option<String> {
        &self.given_encryption_key
    }
    fn get_connect_via(&self) -> &WifiConnectionType {
        &self.connect_via
    }
}

impl WifiConnectOptions {
    #[cfg(test)]
    pub fn from_scan_type(scan_type: ScanType) -> Self {
        Self {
            wifi: WifiOptions::from_scan_type(scan_type),
            ..Self::default()
        }
    }

    // TODO: kill with fire
    pub fn with_synchronous_retry(&self, t: SynchronousRescanType) -> Self {
        Self {
            wifi: self.wifi.with_synchronous_retry(t),
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
    Wifi(WifiScanType)
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
    // TODO: should actually be nmcli
    #[strum(serialize = "networkmanager")]
    NetworkManager,
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
    pub connection_type: WifiConnectionType,
    pub config_data: ConfigData,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConfigData {
    pub config_path: Option<String>,
    //command_run: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionResult {
    pub connection_type: WifiConnectionType,
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
