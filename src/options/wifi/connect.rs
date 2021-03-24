use typed_builder::TypedBuilder;

use crate::options::wifi::WifiOptions;
use crate::prelude::*;

#[derive(Debug, Clone, TypedBuilder)]
pub struct WifiConnectOptions {
    wifi: WifiOptions,
    #[builder(default)]
    auto_mode: AutoMode,
    #[builder(default)]
    connect_via: WifiConnectionType,
    #[builder(default = None)]
    given_essid: Option<String>,
    #[builder(default = false)]
    force_ask_password: bool,
    #[builder(default = None)]
    given_encryption_key: Option<String>,
}

impl Default for WifiConnectOptions {
    fn default() -> Self {
        Self {
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
    fn get_post_parse_context(&self) -> PostParseContext {
        self.wifi.get_post_parse_context()
    }
} 

impl PreParseGlobal for WifiConnectOptions {
    fn d(&self) -> bool {
        self.get_debug()
    }
    fn get_debug(&self) -> bool {
        self.wifi.get_debug()
    }
    fn get_dry_run(&self) -> bool {
        self.wifi.get_dry_run()
    }
    fn get_selection_method(&self) -> &SelectionMethod {
        self.wifi.get_selection_method()
    }
    fn is_test_or_dry_run(&self) -> bool {
        self.wifi.is_test_or_dry_run()
    }
    fn pretend_to_be_root(&self) -> bool {
        self.wifi.pretend_to_be_root()
    }
}

impl Wifi for WifiConnectOptions {
    fn get_scan_type(&self) -> &WifiScanType {
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
    fn get_given_interface_name(&self) -> &Option<String> {
        self.wifi.get_given_interface_name()
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
    pub fn from_scan_type(scan_type: WifiScanType) -> Self {
        Self {
            wifi: WifiOptions::from_scan_type(scan_type),
            ..Self::default()
        }
    }
}
