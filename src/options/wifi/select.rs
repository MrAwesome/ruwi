use typed_builder::TypedBuilder;

use crate::options::interfaces::*;
use crate::structs::*;
use crate::options::wifi::WifiOptions;
use crate::options::GlobalOptions;

#[derive(Debug, Clone, TypedBuilder)]
pub struct WifiSelectOptions {
    #[builder(default)]
    globals: GlobalOptions,
    #[builder(default)]
    wifi: WifiOptions,
    #[builder(default)]
    auto_mode: AutoMode,
}

impl Default for WifiSelectOptions {
    fn default() -> Self {
        Self {
            globals: GlobalOptions::default(),
            wifi: WifiOptions::default(),
            auto_mode: AutoMode::default(),
        }
    }
}

impl Global for WifiSelectOptions {
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

impl LinuxNetworkingInterface for WifiSelectOptions {
    fn get_interface(&self) -> &str {
        self.wifi.get_interface()
    }
}

impl Wifi for WifiSelectOptions {
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

impl AutoSelect for WifiSelectOptions {
    fn get_auto_mode(&self) -> &AutoMode {
        &self.auto_mode
    }
}

