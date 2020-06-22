use crate::rerr;
use std::error::Error;
use std::fmt;

// NOTE: set to pub(crate) temporarily to find unused values
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum RuwiErrorKind {
    BinaryWritableByNonRootWhenRunningAsRoot,
    CmdLineOptParserSafeFailedInTest,
    CommandFindingSpawnError,
    CommandNotFound,
    ErrorReadingNetctlDir,
    FailedToBringLinuxNetworkingInterfaceDown,
    FailedToBringLinuxNetworkingInterfaceUp,
    FailedToConnectViaBluetoothCtl,
    FailedToConnectViaNetctl,
    FailedToConnectViaNetworkManager,
    FailedToFindDevicesWithBluetoothCtl,
    FailedToListKnownNetworksWithNetworkManager,
    FailedToLookForWpaSupplicantProc,
    FailedToPairViaBluetoothCtl,
    FailedToParseIPLinkOutput,
    FailedToParseSelectedLine,
    FailedToRawConnectViaDhclient,
    FailedToRawConnectViaDhcpcd,
    FailedToRawConnectViaNmcli,
    FailedToReadScanResultsFromFile,
    FailedToReadScanResultsFromStdin,
    FailedToRunBluetoothCtlAgentOn,
    FailedToRunBluetoothCtlDefaultAgent,
    FailedToRunBluetoothCtlPowerOn,
    FailedToRunIPLinkShow,
    FailedToRunIWScanAbort,
    FailedToRunIWScanDump,
    FailedToRunIWScanSynchronous,
    FailedToRunIWScanTrigger,
    FailedToRunNmcliScan,
    FailedToRunNmcliScanSynchronous,
    FailedToScanWithBluetoothCtl,
    FailedToScanWithWPACli,
    FailedToSpawnThread,
    FailedToStartBluetoothService,
    FailedToStartNetctl,
    FailedToStartNetworkManager,
    FailedToStartWpaSupplicant,
    FailedToStopNetctl,
    FailedToStopNetworkManager,
    FailedToStopWpaSupplicant,
    FailedToWriteNetctlConfig,
    IWSynchronousScanFailed,
    IWSynchronousScanRanOutOfRetries,
    InvalidNetctlPath,
    InvalidScanTypeAndConnectType,
    InvalidScanTypeAndMethod,
    InvalidServiceIdentifierType,
    InvalidSubcommand,
    KnownNetworksFetchError,
    LoopProtectionMaxExceeded,
    MalformedIWOutput,
    NoInterfaceFoundWithGivenName,
    NoKnownNetworksFound,
    NoMatchingBluetoothDeviceFoundForPrefix,
    NoNetworksFoundMatchingSelectionResult,
    NoNetworksFoundWhenLookingForFirst,
    NoNetworksSeenWithIWScanDump,
    NoNetworksSeenWithWPACliScanResults,
    NoWifiInterfacesFound,
    NoWiredInterfacesFound,
    NotImplementedError,
    OnlyParseCmdlineBailout,
    PromptCommandBailoutRequested,
    PromptCommandFailed,
    PromptCommandSpawnFailed,
    RefreshRequested,
    SingleLinePromptFailed,
    TestDeliberatelyFailedToFindNetworks,
    TestNoNetworksFoundWhenLookingForLast,
    TestNoRefreshOptionFound,
    TestShouldNeverBeSeen,
    TestUsedAutoNoAskWhenNotExpected,
    TestUsedAutoWhenNotExpected,
    TestUsedManualWhenNotExpected,
    UnableToReadMetadataForBinary,
    WPACliHeaderMalformedOrMissing,
}

#[derive(Debug)]
pub struct RuwiError {
    pub kind: RuwiErrorKind,
    pub desc: String,
    pub extra_data: Option<Vec<(String, String)>>,
}

impl Error for RuwiError {}

impl fmt::Display for RuwiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.desc)
    }
}

pub(crate) fn nie<T>(prog: T) -> RuwiError
where
    T: fmt::Debug,
{
    rerr!(
        RuwiErrorKind::NotImplementedError,
        format!("Functionality not implemented: {:?}", prog)
    )
}
