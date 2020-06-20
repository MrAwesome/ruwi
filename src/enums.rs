// For strum macros:
#![allow(clippy::default_trait_access)]

use strum_macros::{AsStaticStr, Display, EnumIter, EnumString};

use crate::interface_management::ip_interfaces::WifiIPInterface;

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
    Wifi(WifiScanType),
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
#[strum(serialize_all = "lowercase")]
pub enum SelectionMethod {
    Dmenu,
    Fzf,
    NoCurses,
}

impl Default for SelectionMethod {
    fn default() -> Self {
        Self::Fzf
    }
}
#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum WifiConnectionType {
    None,
    Netctl,
    Nmcli,
    Print,
}

impl Default for WifiConnectionType {
    fn default() -> Self {
        Self::Netctl
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum WiredConnectionType {
    Netctl,
    Nmcli,
    Dhclient,
    Dhcpcd,
}

impl Default for WiredConnectionType {
    fn default() -> Self {
        Self::Netctl
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum BluetoothConnectionType {
    Bluetoothctl,
}

impl Default for BluetoothConnectionType {
    fn default() -> Self {
        Self::Bluetoothctl
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
    NoneSeen,
    ManuallyRequested,
    Automatic,
}

#[derive(Debug, PartialEq, Eq, EnumIter)]
pub(crate) enum NetworkingService {
    Netctl,
    NetworkManager,
    WpaSupplicant(WifiIPInterface),
    None,
}

// For networks where a service identifier is needed, it can be
// stored here. Otherwise, we just store the type of connection, and use
// the network's public identifier, usually ESSID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkingServiceIdentifier {
    Netctl(String),
    NetworkManager,
}

//Bluetooth(String),

impl NetworkingServiceIdentifier {
    #[cfg(test)]
    pub(crate) fn netctl_nw(essid: &str) -> Option<NetworkingServiceIdentifier> {
        Some(Self::Netctl(essid.to_string()))
    }
}
