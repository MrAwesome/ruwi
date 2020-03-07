use crate::rerr;
use std::error::Error;
use std::fmt;

// TODO: set to pub(crate) temporarily to find unused values
#[derive(Debug, PartialEq, Eq)]
pub enum RuwiErrorKind {
    CommandNotInstalled,
    FailedToBringLinuxNetworkingInterfaceDown,
    FailedToBringLinuxNetworkingInterfaceUp,
    FailedToConnectViaNetctl,
    FailedToConnectViaNetworkManager,
    FailedToConnectViaWPACli,
    FailedToListKnownNetworksWithNetworkManager,
    FailedToParseCommandLine,
    FailedToParseIPLinkOutput,
    FailedToParseSelectedLine,
    FailedToReadScanResultsFromFile,
    FailedToReadScanResultsFromStdin,
    FailedToRawConnectViaDhcpcd,
    FailedToRunCommand,
    FailedToRunIPLinkShow,
    FailedToRunIWDev,
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
    NoInterfacesFoundWithIW,
    NoKnownNetworksFound,
    NoNetworksFoundMatchingSelectionResult,
    NoNetworksSeenWithIWScanDump,
    NoNetworksSeenWithWPACliScanResults,
    NoWifiInterfacesFound,
    NoWiredInterfacesFound,
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
    TestNoNetworksFoundWhenLookingForLast,
    TestNoRefreshOptionFound,
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
    pub extra_data: Option<Vec<(String, String)>>,
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

pub(crate) fn nie<T: fmt::Debug>(prog: T) -> RuwiError {
    rerr!(
        RuwiErrorKind::NotImplementedError,
        format!("Functionality not implemented: {:?}", prog)
    )
}
