use typed_builder::TypedBuilder;

use crate::options::bluetooth::BluetoothOptions;
use crate::prelude::*;

#[derive(Debug, Clone, TypedBuilder)]
pub struct BluetoothConnectOptions {
    bluetooth: BluetoothOptions,
    #[builder(default)]
    auto_mode: AutoMode,
    #[builder(default)]
    connect_via: BluetoothConnectionType,
}

impl BluetoothConnect for BluetoothConnectOptions {
    fn get_connect_via(&self) -> &BluetoothConnectionType {
        &self.connect_via
    }
}

impl AutoSelect for BluetoothConnectOptions {
    fn get_auto_mode(&self) -> &AutoMode {
        &self.auto_mode
    }
}

impl Default for BluetoothConnectOptions {
    fn default() -> Self {
        Self {
            bluetooth: BluetoothOptions::default(),
            connect_via: BluetoothConnectionType::default(),
            auto_mode: AutoMode::default(),
        }
    }
}

impl Global for BluetoothConnectOptions {
    fn d(&self) -> bool {
        self.get_debug()
    }
    fn get_debug(&self) -> bool {
        self.bluetooth.get_debug()
    }
    fn get_dry_run(&self) -> bool {
        self.bluetooth.get_dry_run()
    }
    fn get_selection_method(&self) -> &SelectionMethod {
        self.bluetooth.get_selection_method()
    }
    fn is_test_or_dry_run(&self) -> bool {
        self.bluetooth.is_test_or_dry_run()
    }
    fn pretend_to_be_root(&self) -> bool {
        self.bluetooth.pretend_to_be_root()
    }
}

//impl Bluetooth for BluetoothConnectOptions {
//    fn get_scan_type(&self) -> &BluetoothScanType {
//        self.bluetooth.get_scan_type()
//    }
//    fn get_scan_method(&self) -> &ScanMethod {
//        self.bluetooth.get_scan_method()
//    }
//    fn get_ignore_known(&self) -> bool {
//        self.bluetooth.get_ignore_known()
//    }
//    fn get_force_synchronous_scan(&self) -> bool {
//        self.bluetooth.get_force_synchronous_scan()
//    }
//    fn get_given_interface_name(&self) -> &Option<String> {
//        self.bluetooth.get_given_interface_name()
//    }
//}