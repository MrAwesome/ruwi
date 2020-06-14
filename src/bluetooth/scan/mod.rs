use crate::prelude::*;
use super::BluetoothDevice;

pub(crate) mod bluetoothctl;

pub(crate) trait BluetoothDeviceScanner<O: Global> {
    fn get_opts(&self) -> &O; 
    fn scan(&self, scan_secs: usize) -> Result<(), RuwiError>;
    fn get_devices(&self) -> Result<Vec<BluetoothDevice>, RuwiError>;
}
