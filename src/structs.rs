use std::cmp::Ordering;

type InterfaceName = String;

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
    pub interface: InterfaceName,
    pub output_types: Vec<OutputType>,
    pub connect_via: Option<ConnectionType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanResult {
    pub scan_type: ScanType,
    pub output: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanError {
    NotImplemented,
    DeviceOrResourceBusy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionError {
    NotImplemented,
    FailedToSpawnChildProcessForPrompt,
    FailedToOpenStdinForPrompt,
    FailedToWriteToStdinForPrompt,
    FailedToReadStdoutFromPrompt,
    PromptExitedWithFailure,
    NoMatchingNetworkFromSelection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseResult {
    pub scan_type: ScanType,
    pub seen_networks: Vec<WirelessNetwork>,
    pub line_parse_errors: Vec<(String, IndividualParseError)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    NotImplemented,
    FailedToParse,
    MissingWpaCliHeader,
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

impl Ord for WirelessNetwork {
    fn cmp(&self, other: &Self) -> Ordering {
        self.signal_strength.cmp(&other.signal_strength)
    }
}

impl PartialOrd for WirelessNetwork {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputResult {
    pub output_type: OutputType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputError {
    NotImplemented,
    CouldNotOpenConfigurationFileForWriting,
    CouldNotWriteConfigurationFile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionResult {
    pub connection_type: ConnectionType,
    pub result: Result<String, String>,
    //ipv4_addr: Option<String>,
    //ipv6_addr: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_sorting() {
        let higher_signal = WirelessNetwork {
            essid: "Valparaiso_Guest_House 1".to_string(),
            wpa: true,
            bssid: Some("f4:28:53:fe:a5:d0".to_string()),
            signal_strength: Some(-66),
            channel_utilisation: None,
        };

        let lower_signal = WirelessNetwork {
            essid: "Valparaiso_Guest_House 1".to_string(),
            wpa: true,
            bssid: Some("f4:28:53:fe:a5:d0".to_string()),
            signal_strength: Some(-69),
            channel_utilisation: None,
        };

        let mut wrong_order = vec![higher_signal.clone(), lower_signal.clone()];

        wrong_order.sort();
        let right_order = wrong_order;

        assert_eq![right_order, vec![lower_signal, higher_signal]];
    }

}
