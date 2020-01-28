use crate::run_commands::run_command_pass;
use crate::wpa_cli_initialize::initialize_wpa_supplicant;
use crate::structs::*;

// TODO: implement service killing for when switching between services?
pub(crate) enum NetworkingService {
    Netctl,
    NetworkManager,
    WpaSupplicant,
    None,
}

impl NetworkingService {
    pub(crate) fn start(&self, options: &WifiConnectOptions) -> Result<(), RuwiError> {
        match self {
            Self::Netctl => start_netctl(options),
            Self::NetworkManager => start_networkmanager(options),
            Self::WpaSupplicant => initialize_wpa_supplicant(options),
            Self::None => Ok(()),
        }
    }
}

fn start_netctl(options: &WifiConnectOptions) -> Result<(), RuwiError> {
    run_command_pass(
        options,
        "systemctl",
        &["start", "netctl"],
        RuwiErrorKind::FailedToStartNetctl,
        "Failed to start netctl. Is it installed? Are you running as root?",
    )
}

fn start_networkmanager(options: &WifiConnectOptions) -> Result<(), RuwiError> {
    run_command_pass(
        options,
        "systemctl",
        &["start", "NetworkManager"],
        RuwiErrorKind::FailedToStartNetworkManager,
        "Failed to start NetworkManager. Is it installed? Are you running as root?",
    )
}

pub(crate) trait GetService {
    fn get_service(&self) -> NetworkingService;
}

impl GetService for WifiConnectionType {
    fn get_service(&self) -> NetworkingService {
        match self {
            Self::NetworkManager => NetworkingService::NetworkManager,
            Self::Netctl => NetworkingService::Netctl,
            Self::None | Self::Print => NetworkingService::None,
        }
    }
}

impl GetService for ScanType {
    fn get_service(&self) -> NetworkingService {
        match self {
            Self::Wifi(WifiScanType::Nmcli) => NetworkingService::NetworkManager,
            Self::Wifi(WifiScanType::WpaCli) => NetworkingService::WpaSupplicant,
            Self::Wifi(WifiScanType::IW) | Self::Wifi(WifiScanType::RuwiJSON) => NetworkingService::None,
        }
    }
}
