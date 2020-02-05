// It seems very reasonable for options to be named ...Options
#![allow(clippy::module_name_repetitions)]

// For strum macros:
#![allow(clippy::default_trait_access)]
#![allow(clippy::used_underscore_binding)]

pub mod interfaces;
pub(crate) mod wifi;

use crate::options::interfaces::*;
use crate::options::wifi::connect::WifiConnectOptions;
use crate::options::wifi::select::WifiSelectOptions;
use crate::structs::*;

use strum_macros::{AsStaticStr, Display, EnumIter, EnumString, AsRefStr};
use typed_builder::TypedBuilder;
pub static PROG_NAME: &str = "ruwi";

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, EnumString, EnumIter, Display, AsStaticStr, AsRefStr)]
pub enum RuwiCommand {
    Wifi(RuwiWifiCommand),
    Wired(RuwiWiredCommand),
    Bluetooth(RuwiBluetoothCommand),
}

impl Default for RuwiCommand {
    fn default() -> Self {
        Self::Wifi(RuwiWifiCommand::default())
    }
}

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, EnumString, EnumIter, Display, AsStaticStr, AsRefStr)]
pub enum RuwiWifiCommand {
    Connect(WifiConnectOptions),
    Select(WifiSelectOptions)
    // Select
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
    Connect
}

impl Default for RuwiWiredCommand {
    fn default() -> Self {
        Self::Connect
    }
}

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr, AsRefStr)]
pub enum RuwiBluetoothCommand {
    Pair
}

impl Default for RuwiBluetoothCommand {
    fn default() -> Self {
        Self::Pair
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct GlobalOptions {
    #[builder(default=true)]
    debug: bool,
    #[builder(default=true)]
    dry_run: bool,
    #[builder(default)]
    selection_method: SelectionMethod,
}

impl Global for GlobalOptions {
    fn d(&self) -> bool {
        self.get_debug()
    }
    fn get_debug(&self) -> bool {
        self.debug
    }
    fn get_dry_run(&self) -> bool {
        self.dry_run
    }
    fn get_selection_method(&self) -> &SelectionMethod {
        &self.selection_method
    }
}

impl Default for GlobalOptions {
    fn default() -> Self {
        Self {
            debug: false,
            selection_method: SelectionMethod::default(),
            #[cfg(not(test))]
            dry_run: false,
            #[cfg(test)]
            dry_run: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BluetoothCommandOptions {
}
