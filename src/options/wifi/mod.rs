pub(crate) mod connect;
pub(crate) mod select;

use crate::prelude::*;
use crate::options::GlobalOptions;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct WifiOptions {
    globals: GlobalOptions,
    #[builder(default = None)]
    given_interface_name: Option<String>,
    #[builder(default)]
    scan_type: WifiScanType,
    #[builder(default)]
    scan_method: ScanMethod,
    #[builder(default = false)]
    ignore_known: bool,
    #[builder(default = false)]
    force_synchronous_scan: bool,
}

impl Default for WifiOptions {
    fn default() -> Self {
        Self {
            globals: GlobalOptions::default(),
            scan_type: WifiScanType::default(),
            scan_method: ScanMethod::default(),
            given_interface_name: None,
            ignore_known: false,
            force_synchronous_scan: false,
        }
    }
}

impl WifiOptions {
    #[cfg(test)]
    pub fn from_scan_type(scan_type: WifiScanType) -> Self {
        Self {
            scan_type,
            ..Self::default()
        }
    }
}

impl Wifi for WifiOptions {
    fn get_scan_type(&self) -> &WifiScanType {
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
    fn get_given_interface_name(&self) -> &Option<String> {
        &self.given_interface_name
    }
}

impl Global for WifiOptions {
    fn get_post_parse_context(&self) -> PostParseContext {
        PostParseContext {
            network_or_device: NetworkOrDevice::Network
            
        }
    }
} 

impl PreParseGlobal for WifiOptions {
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
    fn is_test_or_dry_run(&self) -> bool {
        self.globals.is_test_or_dry_run()
    }
    fn pretend_to_be_root(&self) -> bool {
        self.globals.pretend_to_be_root()
    }
}
