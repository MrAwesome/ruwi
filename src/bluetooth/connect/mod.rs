use crate::prelude::*;
use crate::run_commands::SystemCommandRunner;

use super::BluetoothDevice;

impl BluetoothDevice {
    pub(crate) fn connect<O: Global + BluetoothConnect>(&self, opts: &O) -> Result<(), RuwiError> {
        // TODO: you can match here on the connection type

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
}
