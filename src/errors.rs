use std::error::Error;
use std::fmt;
use crate::rerr;

// TODO: set to pub(crate) temporarily to find unused values
#[derive(Debug, PartialEq, Eq)]
pub enum RuwiErrorKind {
    CommandNotInstalled,
    InvalidScanTypeAndMethod,
    FailedToListKnownNetworksWithNetworkManager,
    FailedToBringLinuxNetworkingInterfaceDown,
    FailedToBringLinuxNetworkingInterfaceUp,
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

pub(crate) fn nie<T: fmt::Debug>(prog: T) -> RuwiError {
    rerr!(
        RuwiErrorKind::NotImplementedError,
        format!("Functionality not implemented: {:?}", prog)
    )
}
