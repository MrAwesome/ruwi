// It seems very reasonable for options to be named ...Options
#![allow(clippy::module_name_repetitions)]

// For strum macros:
#![allow(clippy::default_trait_access)]
#![allow(clippy::used_underscore_binding)]

use crate::options::interfaces::*;
use crate::structs::*;

use strum_macros::{AsStaticStr, Display, EnumIter, EnumString, AsRefStr};
use typed_builder::TypedBuilder;
pub static PROG_NAME: &str = "ruwi";

// TODO: refactor this to not expose the raw struct. What should the interface look like?
#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, EnumString, EnumIter, Display, AsStaticStr, AsRefStr)]
pub enum RuwiCommand {
    Wifi(RuwiWifiCommand),
    Wired(RuwiWiredCommand),
    Bluetooth(RuwiBluetoothCommand),
}

impl RuwiCommand {
    // TODO: Obviously, this should return some trait?
    #[cfg(test)]
    pub fn get_options(&self) -> &WifiConnectOptions {
        if let Self::Wifi(RuwiWifiCommand::Connect(opts)) = self {
            opts
        } else {
            todo!("Get rid of this");
        }
    }
}

impl Default for RuwiCommand {
    fn default() -> Self {
        Self::Wifi(RuwiWifiCommand::default())
    }
}

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, EnumString, EnumIter, Display, AsStaticStr, AsRefStr)]
pub enum RuwiWifiCommand {
    Connect(WifiConnectOptions)
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

#[derive(Debug, Clone, TypedBuilder)]
pub struct WifiOptions {
    #[builder(default)]
    scan_type: ScanType,
    #[builder(default)]
    scan_method: ScanMethod,
    #[builder(default="wlan0".to_string())]
    interface: String,
    #[builder(default=false)]
    ignore_known: bool,
    #[builder(default=false)]
    force_synchronous_scan: bool,
}

impl Default for WifiOptions {
    fn default() -> Self {
        Self {
            scan_type: ScanType::default(),
            scan_method: ScanMethod::default(),
            interface: "wlan0".to_string(),
            ignore_known: false,
            force_synchronous_scan: false,
        }
    }
}

impl WifiOptions {
    #[cfg(test)]
    pub fn from_scan_type(scan_type: ScanType) -> Self {
        Self {
            scan_type,
            ..Self::default()
        }
    }
}

impl LinuxNetworkingInterface for WifiOptions {
    fn get_interface(&self) -> &str {
        &self.interface
    }
}

impl Wifi for WifiOptions {
    fn get_scan_type(&self) -> &ScanType {
        &self.scan_type
    }
    fn get_scan_method(&self) -> &ScanMethod {
        &self.scan_method
    }
    fn get_ignore_known(&self) -> bool {
        self.ignore_known
    }
    fn get_force_synchronous_scan(&self) -> bool {
        self.force_synchronous_scan
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct WifiConnectOptions {
    #[builder(default)]
    globals: GlobalOptions,
    #[builder(default)]
    wifi: WifiOptions,
    #[builder(default)]
    auto_mode: AutoMode,
    #[builder(default)]
    connect_via: WifiConnectionType,
    #[builder(default=None)]
    given_essid: Option<String>,
    #[builder(default=false)]
    force_ask_password: bool,
    #[builder(default=None)]
    given_encryption_key: Option<String>,
}

impl Default for WifiConnectOptions {
    fn default() -> Self {
        Self {
            globals: GlobalOptions::default(),
            wifi: WifiOptions::default(),
            connect_via: WifiConnectionType::default(),
            given_essid: None,
            given_encryption_key: None,
            auto_mode: AutoMode::default(),
            force_ask_password: false,
        }
    }
}

impl Global for WifiConnectOptions {
    fn d(&self) -> bool {
        self.get_debug()
    }
    fn get_debug(&self) -> bool {
        self.globals.get_debug()
    }
    fn get_dry_run(&self) -> bool {
        self.globals.get_dry_run()
    }
    fn get_selection_method(&self) -> &SelectionMethod {
        self.globals.get_selection_method()
    }
}

impl LinuxNetworkingInterface for WifiConnectOptions {
    fn get_interface(&self) -> &str {
        self.wifi.get_interface()
    }
}

impl Wifi for WifiConnectOptions {
    fn get_scan_type(&self) -> &ScanType {
        self.wifi.get_scan_type()
    }
    fn get_scan_method(&self) -> &ScanMethod {
        self.wifi.get_scan_method()
    }
    fn get_ignore_known(&self) -> bool {
        self.wifi.get_ignore_known()
    }
    fn get_force_synchronous_scan(&self) -> bool {
        self.wifi.get_force_synchronous_scan()
    }
}

impl WifiConnect for WifiConnectOptions {
    fn get_auto_mode(&self) -> &AutoMode {
        &self.auto_mode
    }
    fn get_force_ask_password(&self) -> bool {
        self.force_ask_password
    }
    fn get_given_essid(&self) -> &Option<String> {
        &self.given_essid
    }
    fn get_given_encryption_key(&self) -> &Option<String> {
        &self.given_encryption_key
    }
    fn get_connect_via(&self) -> &WifiConnectionType {
        &self.connect_via
    }
}

impl WifiConnectOptions {
    #[cfg(test)]
    pub fn from_scan_type(scan_type: ScanType) -> Self {
        Self {
            wifi: WifiOptions::from_scan_type(scan_type),
            ..Self::default()
        }
    }
}


