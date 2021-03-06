use crate::errors::RuwiError;
use crate::options::clear::ClearOptions;
use crate::options::wifi::connect::WifiConnectOptions;
use crate::options::wifi::select::WifiSelectOptions;
use crate::options::wired::connect::WiredConnectOptions;
use crate::options::bluetooth::connect::BluetoothConnectOptions;
use crate::runner::Runner;

use strum_macros::{AsRefStr, AsStaticStr, Display, EnumIter, EnumString};

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, EnumString, EnumIter, Display, AsStaticStr, AsRefStr)]
pub enum RuwiCommand {
    Wifi(RuwiWifiCommand),
    Wired(RuwiWiredCommand),
    Bluetooth(RuwiBluetoothCommand),
    Clear(ClearOptions),
}

impl Default for RuwiCommand {
    fn default() -> Self {
        Self::Wifi(RuwiWifiCommand::default())
    }
}

impl RuwiCommand {
    pub fn run(&self) -> Result<(), RuwiError> {
        // This slightly odd-looking structure is to give us strong typing of
        // our "options" context objects, which each impl the logic for their
        // respective runs. A cleaner-looking alternative to this is a function
        // which returns "Box<dyn Runner>" or such, but that requires heap allocation
        // and loses type info about which options object we're running on.
        match self {
            Self::Wifi(RuwiWifiCommand::Connect(options)) => options.run(),
            Self::Wifi(RuwiWifiCommand::Select(options)) => options.run(),
            Self::Wired(RuwiWiredCommand::Connect(options)) => options.run(),
            Self::Bluetooth(RuwiBluetoothCommand::Connect(options)) => options.run(),
            // TODO: give clear its own options, and make it match this format
            Self::Clear(options) => options.run(),
        }
    }
}

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, EnumString, EnumIter, Display, AsStaticStr, AsRefStr)]
pub enum RuwiWifiCommand {
    Connect(WifiConnectOptions),
    Select(WifiSelectOptions),
}

impl Default for RuwiWifiCommand {
    fn default() -> Self {
        Self::Connect(WifiConnectOptions::default())
    }
}

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, EnumString, EnumIter, Display, AsStaticStr, AsRefStr)]
pub enum RuwiWiredCommand {
    Connect(WiredConnectOptions),
}

impl Default for RuwiWiredCommand {
    fn default() -> Self {
        Self::Connect(WiredConnectOptions::default())
    }
}

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, EnumString, EnumIter, Display, AsStaticStr, AsRefStr)]
pub enum RuwiBluetoothCommand {
    Connect(BluetoothConnectOptions),
}

impl Default for RuwiBluetoothCommand {
    fn default() -> Self {
        Self::Connect(BluetoothConnectOptions::default())
    }
}
