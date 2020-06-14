use crate::prelude::*;
use crate::runner::Runner;

use crate::bluetooth::scan::bluetoothctl::*;
use crate::bluetooth::scan::*;
use crate::bluetooth::BluetoothDevice;
use crate::options::bluetooth::connect::BluetoothConnectOptions;
use crate::utils::loop_check;

use crate::select::Selector;

const LOOP_MAX: u16 = 1000;

impl Runner for BluetoothConnectOptions {
    fn run(&self) -> Result<(), RuwiError> {
        // TODO: cmdline options for bluetooth connect, pair, scan, etc
        // TODO: start bluetooth service with systemctl (create system runner shortcut for this?)
        // TODO: power on, agent on, etc
        eprintln!("First scan: ");
        let dev = scan_and_select_network(self)?;
        dev.connect(self)?;

        Ok(())
    }
}

fn scan_and_select_network<O>(opts: &O) -> Result<BluetoothDevice, RuwiError>
where
    O: Global + AutoSelect,
{
    // TODO: add sync retry flag
    //let mut synchronous_retry = None;
    let mut loop_protection = 0;
    loop {
        loop_check(&mut loop_protection, LOOP_MAX)?;
        let scanner = BluetoothCtlDeviceScanner::new(opts);
        let devs = scanner.get_devices()?;
        if devs.is_empty() {
            let scan_time = 10;
            eprintln!(
                "[ERR] No Bluetooth devices seen! Scanning for {} seconds...",
                scan_time
            );
            scanner.scan(10)?;
        } else {
            return devs.select_network(opts);
        }
    }
}

impl Identifiable for BluetoothDevice {
    fn get_public_name(&self) -> &str {
        &self.get_name()
    }
}

impl Selector<BluetoothDevice> for Vec<BluetoothDevice> {
    fn get_networks(&self) -> &[BluetoothDevice] {
        self
    }
}
