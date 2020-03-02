use typed_builder::TypedBuilder;

use crate::enums::*;
use crate::errors::*;
use crate::options::interfaces::*;
use crate::options::wifi::WifiOptions;

#[derive(Debug, Clone, TypedBuilder)]
pub struct WifiSelectOptions {
    #[builder(default)]
    wifi: WifiOptions,
    #[builder(default)]
    auto_mode: AutoMode,
}

impl Default for WifiSelectOptions {
    fn default() -> Self {
        Self {
            wifi: WifiOptions::default(),
            auto_mode: AutoMode::default(),
        }
    }
}

impl AutoSelect for WifiSelectOptions {
    fn get_auto_mode(&self) -> &AutoMode {
        &self.auto_mode
    }
}

impl Global for WifiSelectOptions {
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
}

impl LinuxNetworkingInterface for WifiSelectOptions {
    fn get_interface_name(&self) -> &str {
        self.wifi.get_interface_name()
    }
    fn bring_interface_up(&self) -> Result<(), RuwiError> {
        self.wifi.bring_interface_up()
    }
    fn bring_interface_down(&self) -> Result<(), RuwiError> {
        self.wifi.bring_interface_down()
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
