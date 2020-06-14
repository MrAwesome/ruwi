use crate::prelude::*;
use crate::runner::Runner;

use crate::bluetooth::scan::*;
use crate::bluetooth::scan::bluetoothctl::*;
use crate::options::GlobalOptions;
use crate::options::bluetooth::connect::BluetoothConnectOptions;

impl Runner for BluetoothConnectOptions {
    fn run(&self) -> Result<(), RuwiError> {
        // TODO: cmdline options for bluetooth connect, pair, scan, etc

        let opts = GlobalOptions::default();
        let scanner = BluetoothCtlDeviceScanner::new(&opts);
        let devs = scanner.get_devices()?;
        eprintln!("First scan: ");
        dbg!(devs);
        scanner.scan(10)?;
        let devs = scanner.get_devices()?;
        eprintln!("Second scan: ");
        dbg!(devs);

        Ok(())
    }
}
