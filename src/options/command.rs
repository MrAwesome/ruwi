use crate::enums::NetworkingService;
use crate::enums::*;
use crate::errors::*;
use crate::options::interfaces::*;
use crate::options::wifi::connect::WifiConnectOptions;
use crate::options::wifi::select::WifiSelectOptions;
use crate::options::GlobalOptions;
use crate::rerr;
use crate::runner::Runner;

use strum_macros::{AsRefStr, AsStaticStr, Display, EnumIter, EnumString};

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, EnumString, EnumIter, Display, AsStaticStr, AsRefStr)]
pub enum RuwiCommand {
    Wifi(RuwiWifiCommand),
    Wired(RuwiWiredCommand),
    Bluetooth(RuwiBluetoothCommand),
    Clear(GlobalOptions),
}

impl Default for RuwiCommand {
    fn default() -> Self {
        Self::Wifi(RuwiWifiCommand::default())
    }
}

impl RuwiCommand {
    pub fn run(&self) -> Result<(), RuwiError> {
        self.verify_options()?;

        match self {
            Self::Wifi(RuwiWifiCommand::Connect(options)) => options.run(),
            Self::Wifi(RuwiWifiCommand::Select(options)) => options.run(),
            Self::Wired(RuwiWiredCommand::Connect) => unimplemented!(),
            Self::Bluetooth(RuwiBluetoothCommand::Pair) => unimplemented!(),
            Self::Clear(options) => NetworkingService::stop_all(options),
        }
    }

    fn verify_options(&self) -> Result<(), RuwiError> {
        match self {
            Self::Wifi(RuwiWifiCommand::Connect(options)) => {
                if options.get_scan_method() == &ScanMethod::ByRunning
                    && options.get_connect_via() == &WifiConnectionType::Nmcli
                    && options.get_scan_type() != &ScanType::Wifi(WifiScanType::Nmcli)
                {
                    Err(rerr!(
                            RuwiErrorKind::InvalidScanTypeAndConnectType,
                            "Non-nmcli scan types do not work when connect_via is set to nmcli, as nmcli needs the NetworkManager service enabled while it looks for known networks. You can pass in results from another scanning program with -I or -F, but most likely you just want to add \"-s nmcli\" to wifi."

                    ))
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
}

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, EnumString, EnumIter, Display, AsStaticStr, AsRefStr)]
pub enum RuwiWifiCommand {
    Connect(WifiConnectOptions),
    Select(WifiSelectOptions), // Select
                               // JSON
}

impl Default for RuwiWifiCommand {
    fn default() -> Self {
        Self::Connect(WifiConnectOptions::default())
    }
}

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr, AsRefStr)]
pub enum RuwiWiredCommand {
    Connect,
}

impl Default for RuwiWiredCommand {
    fn default() -> Self {
        Self::Connect
    }
}

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr, AsRefStr)]
pub enum RuwiBluetoothCommand {
    Pair,
}

impl Default for RuwiBluetoothCommand {
    fn default() -> Self {
        Self::Pair
    }
}
