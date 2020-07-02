mod connect;
pub(crate) mod blurz;
mod bluetoothctl;
// mod disconnect;
// mod pair;
pub(crate) mod utils;
// mod service_management;

use crate::prelude::*;
use crate::run_commands::SystemCommandRunner;
use typed_builder::TypedBuilder;

// TODO: strum, add to cmdline, use based on that
// TODO: agent on, pairable on, power on, etc
// TODO: if bluetoothctl devices shows anything, just open that up for selection
// TODO: trust device, pair device, connect to device
// TODO: synchronous rescan logic during selection, or when devices returns nothing

// TODO: make transparent
pub(crate) use bluetoothctl::BluetoothCtlController as TODOBluetoothCtlController;

pub(crate) trait BluetoothService where Self: Sized {
    type Opts: Global + Sized;
    fn get_opts(&self) -> &Self::Opts;
    fn list_devices(&self) -> Result<Vec<BluetoothDevice>, RuwiError>;
    fn power_on(&self) -> Result<(), RuwiError>;
    fn power_off(&self) -> Result<(), RuwiError>;
    fn scan(&self, scan_secs: usize) -> Result<(), RuwiError>;

    fn start_bluetooth_service(&self) -> Result<(), RuwiError> {
        SystemCommandRunner::new(self.get_opts(), "systemctl", &["start", "bluetooth"])
            .run_command_pass(
                RuwiErrorKind::FailedToStartBluetoothService,
                "Failed to start the bluetooth service!",
            )
    }
    fn stop_bluetooth_service(&self) -> Result<(), RuwiError> {
        SystemCommandRunner::new(self.get_opts(), "systemctl", &["stop", "bluetooth"])
            .run_command_pass(
                RuwiErrorKind::FailedToStopBluetoothService,
                "Failed to start the bluetooth service!",
            )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BluetoothKnownDeviceIdentifier {
    Blurz(String)
}

#[derive(Debug, Clone, TypedBuilder, Eq, PartialEq)]
pub(crate) struct BluetoothDevice {
    name: BluetoothDeviceName,
    addr: BluetoothDeviceAddress,
    #[builder(default = None)]
    known_identifier: Option<BluetoothKnownDeviceIdentifier>,
}

string_container! {BluetoothDeviceName, BluetoothDeviceAddress}

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
