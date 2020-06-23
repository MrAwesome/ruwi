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
    TestError,
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
    pub exit_code: Option<i32>,
}

impl Error for RuwiError {}

impl fmt::Display for RuwiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.desc)
    }
}

impl RuwiError {
    pub fn get_linux_exit_code(&self) -> i32 {
        match self.exit_code {
            Some(code) => code,
            None => 1,
        }
    }

    pub fn print_error(&self) {
        eprintln!("[ERR]: Error encountered! ({:?})", self.kind);
        eprintln!("[ERR]: {}", self);
        if let Some(extra_data) = &self.extra_data {
            for (key, val) in extra_data {
                eprintln!("* {}: {}", key, val);
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_default_exit_code_1() {
        let err = rerr!(RuwiErrorKind::TestError, "A TEST ERROR");
        let exit_code = err.get_linux_exit_code();
        assert_eq![exit_code, 1];
    }

    #[test]
    fn check_custom_exit_code() {
        let mut err = rerr!(RuwiErrorKind::TestError, "A TEST ERROR");
        err.exit_code = Some(12);
        let exit_code = err.get_linux_exit_code();
        assert_eq![exit_code, 12];
    }
}
