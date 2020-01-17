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
    pub(crate) fn start(&self, options: &Options) -> Result<(), RuwiError> {
        match self {
            Self::Netctl => start_netctl(options),
            Self::NetworkManager => start_networkmanager(options),
            Self::WpaSupplicant => initialize_wpa_supplicant(options),
            Self::None => Ok(()),
        }
    }
}

fn start_netctl(options: &Options) -> Result<(), RuwiError> {
    run_command_pass(
        options.debug,
        "systemctl",
        &["start", "netctl"],
        RuwiErrorKind::FailedToStartNetctl,
        "Failed to start netctl. Is it installed? Are you running as root?",
    )
}

fn start_networkmanager(options: &Options) -> Result<(), RuwiError> {
    run_command_pass(
        options.debug,
        "systemctl",
        &["start", "NetworkManager"],
        RuwiErrorKind::FailedToStartNetworkManager,
        "Failed to start NetworkManager. Is it installed? Are you running as root?",
    )
}

pub(crate) trait GetService {
    fn get_service(&self) -> NetworkingService;
}

impl GetService for ConnectionType {
    fn get_service(&self) -> NetworkingService {
        match self {
            Self::NetworkManager => NetworkingService::NetworkManager,
            Self::Netctl => NetworkingService::Netctl,
            Self::None | Self::Print => NetworkingService::None,
        }
    }
}

impl GetService for WifiScanType {
    fn get_service(&self) -> NetworkingService {
        match self {
            Self::Nmcli => NetworkingService::NetworkManager,
            Self::WpaCli => NetworkingService::WpaSupplicant,
            Self::IW | Self::RuwiJSON => NetworkingService::None,
        }
    }
}
