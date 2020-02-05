pub(crate) mod connect;
pub(crate) mod select;

use typed_builder::TypedBuilder;
use crate::structs::*;
use crate::options::interfaces::*;

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
