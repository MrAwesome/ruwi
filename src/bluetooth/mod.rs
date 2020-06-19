mod connect;
// mod disconnect;
// mod pair;
pub(crate) mod scan;
// mod service_management;

use crate::prelude::*;
use typed_builder::TypedBuilder;

// TODO: agent on, pairable on, power on, etc
// TODO: if bluetoothctl devices shows anything, just open that up for selection
// TODO: trust device, pair device, connect to device
// TODO: synchronous rescan logic during selection, or when devices returns nothing

#[derive(Debug, Clone, TypedBuilder, Eq, PartialEq)]
pub(crate) struct BluetoothDevice {
    name: String,
    addr: String,
    #[builder(default = None)]
    service_identifier: Option<NetworkServiceIdentifier>,
}

impl BluetoothDevice {
    pub(crate) fn get_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn get_addr(&self) -> &str {
        &self.addr
    }
}

impl Known for BluetoothDevice {
    fn is_known(&self) -> bool {
        self.service_identifier.is_some()
    }
    fn get_service_identifier(&self) -> Option<&NetworkServiceIdentifier> {
        self.service_identifier.as_ref()
    }
}
