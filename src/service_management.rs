use crate::errors::*;
use crate::options::interfaces::*;
use crate::run_commands::run_command_pass;
use crate::structs::*;
use crate::wpa_cli_initialize::*;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// TODO: implement service killing for when switching between services?

#[derive(Debug, EnumIter)]
pub(crate) enum NetworkingService {
    Netctl,
    NetworkManager,
    WpaSupplicant,
    None,
}

impl NetworkingService {
    pub(crate) fn start<O>(&self, options: &O) -> Result<(), RuwiError>
    where
        O: Global,
    {
        match self {
            Self::Netctl => start_netctl(options),
            Self::NetworkManager => start_networkmanager(options),
            Self::WpaSupplicant => initialize_wpa_supplicant(options),
            Self::None => Ok(()),
        }
    }

    pub(crate) fn stop_all<O: 'static>(options: &O) -> Result<(), RuwiError>
    where
        O: Global + Send + Sync + Clone,
    {
        use std::thread;
        let options: &'static O = Box::leak(Box::new(options.clone()));
        let handles: Vec<_> = Self::iter()
            .map(|service| thread::spawn(move || service.stop(options)))
            .collect();
        for h in handles {
            let res = h
                .join()
                .expect("Failure joining thread while stopping networking services!");
            match res {
                Ok(()) => (),
                Err(err) => eprintln!("[ERR]: {}", err),
            }
        }

        Ok(())
    }

    pub(crate) fn stop<O>(&self, options: &O) -> Result<(), RuwiError>
    where
        O: Global,
    {
        match self {
            Self::Netctl => stop_netctl(options),
            Self::NetworkManager => stop_networkmanager(options),
            Self::WpaSupplicant => kill_wpa_supplicant(options),
            Self::None => Ok(()),
        }
    }
}

fn start_netctl<O>(options: &O) -> Result<(), RuwiError>
where
    O: Global,
{
    run_command_pass(
        options,
        "systemctl",
        &["start", "netctl"],
        RuwiErrorKind::FailedToStartNetctl,
        "Failed to start netctl. Is it installed? Are you running as root?",
    )
}

fn stop_netctl<O>(options: &O) -> Result<(), RuwiError>
where
    O: Global,
{
    run_command_pass(
        options,
        "systemctl",
        &["stop", "netctl"],
        RuwiErrorKind::FailedToStopNetctl,
        "Failed to stop netctl. Are you running as root?",
    )
}

fn start_networkmanager<O>(options: &O) -> Result<(), RuwiError>
where
    O: Global,
{
    run_command_pass(
        options,
        "systemctl",
        &["start", "NetworkManager"],
        RuwiErrorKind::FailedToStartNetworkManager,
        "Failed to start NetworkManager. Is it installed? Are you running as root?",
    )
}

fn stop_networkmanager<O>(options: &O) -> Result<(), RuwiError>
where
    O: Global,
{
    run_command_pass(
        options,
        "systemctl",
        &["stop", "NetworkManager"],
        RuwiErrorKind::FailedToStopNetworkManager,
        "Failed to stop NetworkManager. Are you running as root?",
    )
}

pub(crate) trait GetService {
    fn get_service(&self) -> NetworkingService;
}

impl GetService for WifiConnectionType {
    fn get_service(&self) -> NetworkingService {
        match self {
            Self::Nmcli => NetworkingService::NetworkManager,
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
            Self::Wifi(WifiScanType::IW) | Self::Wifi(WifiScanType::RuwiJSON) => {
                NetworkingService::None
            }
        }
    }
}
