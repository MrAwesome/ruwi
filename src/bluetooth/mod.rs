mod connect;
// mod disconnect;
// mod pair;
pub(crate) mod utils;
pub(crate) mod scan;
// mod service_management;

use crate::prelude::*;
use typed_builder::TypedBuilder;

// TODO: agent on, pairable on, power on, etc
// TODO: if bluetoothctl devices shows anything, just open that up for selection
// TODO: trust device, pair device, connect to device
// TODO: synchronous rescan logic during selection, or when devices returns nothing

string_container! {BluetoothDeviceName, BluetoothDeviceAddress}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BluetoothKnownDeviceIdentifier {}

#[derive(Debug, Clone, TypedBuilder, Eq, PartialEq)]
pub(crate) struct BluetoothDevice {
    name: BluetoothDeviceName,
    addr: BluetoothDeviceAddress,
    #[builder(default = None)]
    known_identifier: Option<BluetoothKnownDeviceIdentifier>,
}

impl fmt::Display for BluetoothDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{} ({})", self.name.as_ref(), self.addr.as_ref()))
    }
}

impl BluetoothDevice {
    pub(crate) fn get_name(&self) -> &str {
        &self.name.as_ref()
    }

    pub(crate) fn get_addr(&self) -> &str {
        &self.addr.as_ref()
    }
}

impl Known for BluetoothDevice {
    type ServiceIdentifier = BluetoothKnownDeviceIdentifier;

    fn is_known(&self) -> bool {
        self.known_identifier.is_some()
    }
    fn get_service_identifier(&self) -> Option<&BluetoothKnownDeviceIdentifier> {
        self.known_identifier.as_ref()
    }
}
