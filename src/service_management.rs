use crate::prelude::*;
use crate::interface_management::ip_interfaces::WifiIPInterface;
use crate::run_commands::SystemCommandRunner;
use crate::wpa_cli_initialize::{initialize_wpa_supplicant, kill_wpa_supplicant};

use strum::IntoEnumIterator;

// TODO: check systemctl is-active before starting

// TODO: use MockAll and ensure the correct services are started in the correct places in
// raw_interface_connect, etc
impl NetworkingService {
    pub(crate) fn start<O>(&self, options: &O) -> Result<(), RuwiError>
    where
        O: Global,
    {
        match self {
            Self::Netctl => start_netctl(options),
            Self::NetworkManager => start_networkmanager(options),
            Self::WpaSupplicant(interface) => initialize_wpa_supplicant(options, interface),
            Self::None => Ok(()),
        }
    }

    pub(crate) fn stop<O>(&self, options: &O) -> Result<(), RuwiError>
    where
        O: Global,
    {
        match self {
            Self::Netctl => stop_netctl(options),
            Self::NetworkManager => stop_networkmanager(options),
            Self::WpaSupplicant(_) => kill_wpa_supplicant(options),
            Self::None => Ok(()),
        }
    }

    pub(crate) fn stop_all<O: 'static>(options: &O) -> Result<(), RuwiError>
    where
        O: Global + Send + Sync + Clone,
    {
        use std::thread;
        let options: &'static O = Box::leak(Box::new(options.clone()));
        let all_service_types = Self::iter();
        let handles: Vec<_> = all_service_types
            .map(|service| thread::spawn(move || service.stop(options)))
            .collect();
        for h in handles {
            let res = h
                .join()
                .expect("Failure joining thread while stopping networking services!");
            match res {
                Ok(()) => (),
                Err(err) => err.print_error(),
            }
        }

        Ok(())
    }
}

fn start_netctl<O>(options: &O) -> Result<(), RuwiError>
where
    O: Global,
{
    SystemCommandRunner::new(options, "systemctl", &["start", "netctl"]).run_command_pass(
        RuwiErrorKind::FailedToStartNetctl,
        "Failed to start netctl. Is it installed? Are you running as root?",
    )
}

fn stop_netctl<O>(options: &O) -> Result<(), RuwiError>
where
    O: Global,
{
    SystemCommandRunner::new(options, "netctl", &["stop-all"]).run_command_pass(
        RuwiErrorKind::FailedToStopNetctl,
        "Failed to stop netctl. Are you running as root?",
    )?;
    SystemCommandRunner::new(options, "systemctl", &["stop", "netctl"]).run_command_pass(
        RuwiErrorKind::FailedToStopNetctl,
        "Failed to stop netctl. Are you running as root?",
    )?;
    Ok(())
}

fn start_networkmanager<O>(options: &O) -> Result<(), RuwiError>
where
    O: Global,
{
    SystemCommandRunner::new(options, "systemctl", &["start", "NetworkManager"]).run_command_pass(
        RuwiErrorKind::FailedToStartNetworkManager,
        "Failed to start NetworkManager. Is it installed? Are you running as root?",
    )
}

fn stop_networkmanager<O>(options: &O) -> Result<(), RuwiError>
where
    O: Global,
{
    SystemCommandRunner::new(options, "systemctl", &["stop", "NetworkManager"]).run_command_pass(
        RuwiErrorKind::FailedToStopNetworkManager,
        "Failed to stop NetworkManager. Are you running as root?",
    )
}

impl GetService for WifiConnectionType {
    fn get_service(&self, _interface: Option<&WifiIPInterface>) -> NetworkingService {
        match self {
            Self::Nmcli => NetworkingService::NetworkManager,
            Self::Netctl => NetworkingService::Netctl,
            Self::None | Self::Print => NetworkingService::None,
        }
    }
}

impl GetService for WifiScanType {
    fn get_service(&self, interface: Option<&WifiIPInterface>) -> NetworkingService {
        match self {
            WifiScanType::Nmcli => NetworkingService::NetworkManager,
            WifiScanType::WpaCli => NetworkingService::WpaSupplicant(
                interface
                    .expect("Interface must be provided to wpa_supplicant!")
                    .clone(),
            ),
            WifiScanType::IW | WifiScanType::RuwiJSON => NetworkingService::None,
        }
    }
}
