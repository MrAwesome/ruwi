pub(crate) mod connect;
pub(crate) mod select;

use crate::enums::*;
use crate::options::interfaces::*;
use crate::options::GlobalOptions;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct WifiOptions {
    globals: GlobalOptions,
    #[builder(default = None)]
    given_interface_name: Option<String>,
    #[builder(default)]
    scan_type: ScanType,
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
            scan_type: ScanType::default(),
            scan_method: ScanMethod::default(),
            given_interface_name: None,
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
    fn get_given_interface_name(&self) -> &Option<String> {
        &self.given_interface_name
    }
}

impl Global for WifiOptions {
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
