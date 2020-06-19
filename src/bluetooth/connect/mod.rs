use crate::prelude::*;
use crate::run_commands::SystemCommandRunner;

use super::BluetoothDevice;

impl BluetoothDevice {
    pub(crate) fn connect<O: Global + BluetoothConnect>(&self, opts: &O) -> Result<(), RuwiError> {
        match opts.get_connect_via() {
            BluetoothConnectionType::Bluetoothctl => {
                SystemCommandRunner::new(opts, "bluetoothctl", &["connect", self.get_addr()])
                    .run_command_pass(
                        RuwiErrorKind::FailedToConnectViaBluetoothCtl,
                        &format!(
                            "Failed to connect to {} ({}) using `bluetoothctl connect`!",
                            self.get_name(),
                            self.get_addr()
                        ),
                    )
            }
        }
    }

    pub(crate) fn pair<O: Global + BluetoothConnect>(&self, opts: &O) -> Result<(), RuwiError> {
        match opts.get_connect_via() {
            BluetoothConnectionType::Bluetoothctl => {
                let todo = "pairing with passcode";
                // TODO: make this work with stdin, or just take a passcode on the command line, or
                // both (what about dmenu? maybe try to detect if a key is needed and prompt for it
                // using selection mechanism)
                SystemCommandRunner::new(opts, "bluetoothctl", &["pair", self.get_addr()])
                    .run_command_pass(
                        RuwiErrorKind::FailedToPairViaBluetoothCtl,
                        &format!(
                            "Failed to connect to {} ({}) using `bluetoothctl connect`!",
                            self.get_name(),
                            self.get_addr()
                        ),
                    )
            }
        }
    }
}
