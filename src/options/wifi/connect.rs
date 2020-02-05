use typed_builder::TypedBuilder;

use crate::options::interfaces::*;
use crate::structs::*;
use crate::options::wifi::WifiOptions;
use crate::options::GlobalOptions;

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

impl AutoSelect for WifiConnectOptions {
    fn get_auto_mode(&self) -> &AutoMode {
        &self.auto_mode
    }
}

impl WifiConnect for WifiConnectOptions {
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
