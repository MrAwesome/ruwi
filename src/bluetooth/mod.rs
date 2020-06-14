// mod disconnect;
// mod pair;
pub(crate) mod scan;
// mod service_management;

use typed_builder::TypedBuilder;

// TODO: agent on, pairable on, power on, etc
// TODO: if bluetoothctl devices shows anything, just open that up for selection
// TODO: trust device, pair device, connect to device
// TODO: synchronous rescan logic during selection, or when devices returns nothing

#[derive(Debug, TypedBuilder)]
pub(crate) struct BluetoothDevice {
    name: String,
    addr: String,
}

impl BluetoothDevice {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_addr(&self) -> &str {
        &self.addr
    }
}
