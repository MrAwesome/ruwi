use crate::enums::NetworkingService;
use crate::errors::*;
use crate::options::wifi::connect::WifiConnectOptions;
use crate::options::wifi::select::WifiSelectOptions;
use crate::options::GlobalOptions;
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
        match self {
            Self::Wifi(RuwiWifiCommand::Connect(options)) => options.run(),
            Self::Wifi(RuwiWifiCommand::Select(options)) => options.run(),
            Self::Wired(RuwiWiredCommand::Connect) => unimplemented!(),
            Self::Bluetooth(RuwiBluetoothCommand::Pair) => unimplemented!(),
            Self::Clear(options) => NetworkingService::stop_all(options),
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
