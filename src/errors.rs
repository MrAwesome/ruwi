use crate::rerr;
use std::error::Error;
use std::fmt;

// TODO: set to pub(crate) temporarily to find unused values
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum RuwiErrorKind {
    CommandNotFound,
    BinaryWritableByNonRootWhenRunningAsRoot,
    FailedToBringLinuxNetworkingInterfaceDown,
    FailedToBringLinuxNetworkingInterfaceUp,
    FailedToConnectViaNetctl,
    FailedToConnectViaNetworkManager,
    FailedToListKnownNetworksWithNetworkManager,
    FailedToParseIPLinkOutput,
    FailedToParseSelectedLine,
    FailedToReadScanResultsFromFile,
    FailedToReadScanResultsFromStdin,
    FailedToRawConnectViaDhcpcd,
    FailedToRawConnectViaDhclient,
    FailedToRawConnectViaNmcli,
    FailedToRunIPLinkShow,
    FailedToRunIWScanAbort,
    FailedToRunIWScanDump,
    FailedToRunIWScanSynchronous,
    FailedToRunIWScanTrigger,
    FailedToRunNmcliScan,
    FailedToRunNmcliScanSynchronous,
    FailedToScanWithWPACli,
    FailedToSpawnThread,
    FailedToStartNetctl,
    FailedToStartNetworkManager,
    FailedToStartWpaSupplicant,
    FailedToStopNetctl,
    FailedToStopNetworkManager,
    FailedToStopWpaSupplicant,
    FailedToWriteNetctlConfig,
    IWSynchronousScanFailed,
    IWSynchronousScanRanOutOfRetries,
    InvalidScanTypeAndConnectType,
    InvalidScanTypeAndMethod,
    InvalidSubcommand,
    KnownNetworksFetchError,
    LoopProtectionMaxExceeded,
    MalformedIWOutput,
    NoInterfaceFoundWithGivenName,
    NoKnownNetworksFound,
    NoNetworksFoundMatchingSelectionResult,
    NoNetworksSeenWithIWScanDump,
    NoNetworksSeenWithWPACliScanResults,
    NoWifiInterfacesFound,
    NoWiredInterfacesFound,
    NotImplementedError,
    OnlyParseCmdlineBailout,
    PromptCommandFailed,
    PromptCommandSpawnFailed,
    RefreshRequested,
    SingleLinePromptFailed,
    StepRunnerLoopPreventionCapExceeded,
    TestCmdLineOptParserSafeFailed,
    TestDeliberatelyFailedToFindNetworks,
    TestNoNetworksFoundWhenLookingForFirst,
    TestNoNetworksFoundWhenLookingForLast,
    TestNoRefreshOptionFound,
    TestShouldNeverBeSeen,
    TestUsedAutoNoAskWhenNotExpected,
    TestUsedAutoWhenNotExpected,
    TestUsedManualWhenNotExpected,
    UsedTerminalStep,
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

pub(crate) fn nie<T: fmt::Debug>(prog: T) -> RuwiError {
    rerr!(
        RuwiErrorKind::NotImplementedError,
        format!("Functionality not implemented: {:?}", prog)
    )
}
