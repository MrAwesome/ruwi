use crate::prelude::*;
use crate::runner::Runner;

use crate::bluetooth::scan::bluetoothctl::*;
use crate::bluetooth::scan::*;
use crate::bluetooth::BluetoothDevice;
use crate::options::bluetooth::connect::BluetoothConnectOptions;
use crate::synchronous_retry_logic::{
    manual_refresh_requested, should_auto_retry_with_synchronous_scan,
};
use crate::utils::loop_check;

use crate::select::Selector;

const LOOP_MAX: u16 = 1000;

impl Runner for BluetoothConnectOptions {
    fn run(&self) -> Result<(), RuwiError> {
        // TODO: cmdline options for bluetooth connect, pair, scan, etc
        // TODO: start bluetooth service with systemctl (create system runner shortcut for this?)
        // TODO: power on, agent on, etc
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
    // TODO: remove code duplication between this and src/runner/wifi/utils.rs
    let mut synchronous_retry = None;
    let mut loop_protection = 0;
    loop {
        loop_check(&mut loop_protection, LOOP_MAX)?;

        // TODO: generic scanner based on value of scan_via
        let scanner = BluetoothCtlDeviceScanner::new(opts);
        let devs = scanner.get_devices()?;

        if synchronous_retry.is_some() {
            eprintln!("[NOTE]: Scanning for bluetooth devices...");
            scanner.scan(10)?;
        }

        // NOTE: this logic could reside in the scanner itself
        if devs.is_empty() && loop_protection < 3 {
            synchronous_retry = Some(SynchronousRescanType::NoneSeen);
            continue;
        }

        if should_auto_retry_with_synchronous_scan(opts, &devs, &synchronous_retry) {
            synchronous_retry = Some(SynchronousRescanType::Automatic);
            continue;
        }

        let dev = devs.select_network(opts);

        if manual_refresh_requested(&dev) {
            synchronous_retry = Some(SynchronousRescanType::ManuallyRequested);
            continue;
        }

        return dev;
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
