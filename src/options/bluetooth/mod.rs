pub(crate) mod connect;

use crate::prelude::*;
use crate::options::GlobalOptions;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct BluetoothOptions {
    globals: GlobalOptions,
    //#[builder(default = None)]
    //given_interface_name: Option<String>,
    //#[builder(default)]
    //scan_type: BluetoothScanType,
    //#[builder(default)]
    //scan_method: ScanMethod,
    //#[builder(default = false)]
    //force_synchronous_scan: bool,
}

impl Default for BluetoothOptions {
    fn default() -> Self {
        Self {
            globals: GlobalOptions::default(),
            //scan_type: BluetoothScanType::default(),
            //scan_method: ScanMethod::default(),
            //given_interface_name: None,
            //ignore_known: false,
            //force_synchronous_scan: false,
        }
    }
}

impl Global for BluetoothOptions {
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

//impl Bluetooth for BluetoothOptions {
//    fn get_scan_type(&self) -> &BluetoothScanType {
//        &self.scan_type
//    }
//    fn get_scan_method(&self) -> &ScanMethod {
//        &self.scan_method
//    }
//    fn get_ignore_known(&self) -> bool {
//        self.ignore_known
//    }
//    fn get_force_synchronous_scan(&self) -> bool {
//        self.force_synchronous_scan
//    }
//    fn get_given_interface_name(&self) -> &Option<String> {
//        &self.given_interface_name
//    }
//}
