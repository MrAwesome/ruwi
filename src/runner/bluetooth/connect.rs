use crate::prelude::*;
use crate::runner::Runner;

use crate::bluetooth::{TODOBluetoothCtlController, BluetoothService};
use crate::bluetooth::utils::get_first_matching_device;
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
        // TODO: make prefix matching work for essids as well
        // TODO: cmdline options for bluetooth connect, pair, scan, etc
        // TODO: integration and unit tests for bluetoothctl scanning
        // TODO: read from file/stdin, so you can do integration tests
        // TODO(high): bluetoothctl paired-devices as known
        // TODO: bluetoothctl disconnect / clear
        // TODO(wishlist): refactor all of this runner / datagatherer logic?
        // TODO(high): start bluetooth service with systemctl (create system runner shortcut for this?)
        // TODO(critical): power on, agent on, etc
        //
        //
        // use service for scanning? or service for connection? or just start up before both if necessary?
        // TODO: capture output of "scan on" and use for new devices, "devices" gives known devices
        todo_bluetoothctl_startup_bluetooth_stack(self)?;
        // TODO: unit or integration test
        let dev = if let Some(addr) = self.get_given_device_addr() {
            BluetoothDevice::builder()
                .addr(addr)
                .name("<Anonymous device with given address>")
                .build()
        } else {
            scan_and_select_device(self)?
        };
        // TODO: make pair work with stdin if pairing is needed (check devices output)
        // TODO: mark devices as known if seen in devices output, and don't pair with them
        // TODO: you can use rexpect for pairing via bluetoothctl to detect, if necessary
        //dev.pair(self)?;
        // TODO: take optional device name or addr (one field for either?)
        dev.connect(self)?;

        // TODO: include pulseaudio/pulsemixer/etc instructions? or handle that in Ruwi?
        // TODO: --use-audio?

        Ok(())
    }
}

// TODO: integration test reading devices from file and "connecting" to one with this
// TODO: move this up a level, it's not connect-specific
fn todo_bluetoothctl_startup_bluetooth_stack<O>(opts: &O) -> Result<(), RuwiError>
where
    O: Global,
{
    use crate::run_commands::SystemCommandRunner;
    // TODO: generic systemctl unit starter?
    SystemCommandRunner::new(opts, "systemctl", &["start", "bluetooth"]).run_command_pass(
        RuwiErrorKind::FailedToStartBluetoothService,
        "Failed to start the bluetooth service!",
    )?;
    SystemCommandRunner::new(opts, "bluetoothctl", &["power", "on"]).run_command_pass(
        RuwiErrorKind::FailedToRunBluetoothCtlPowerOn,
        "Failed to power on bluetooth device using bluetoothctl!",
    )?;
    // TODO: learn more about this and how/whether to set it up
    //SystemCommandRunner::new(opts, "bluetoothctl", &["agent", "on"]).run_command_pass(
    //    RuwiErrorKind::FailedToRunBluetoothCtlAgentOn,
    //    "Failed to turn on bluetooth agent using bluetoothctl!",
    //)
    //SystemCommandRunner::new(opts, "bluetoothctl", &["default-agent"]).run_command_pass(
    //    RuwiErrorKind::FailedToRunBluetoothCtlDefaultAgent,
    //    "Failed to power on bluetooth device using bluetoothctl!",
    //)
    Ok(())
}

fn scan_and_select_device<O>(opts: &O) -> Result<BluetoothDevice, RuwiError>
where
    O: Global + AutoSelect + BluetoothConnect, // TODO: Bluetooth?
{
    // TODO: add sync retry flag
    // TODO: remove code duplication between this and src/runner/wifi/utils.rs
    let mut synchronous_retry = None;
    let mut loop_protection = 0;
    loop {
        loop_check(&mut loop_protection, LOOP_MAX)?;

        // TODO: generic scanner based on value of scan_via
        let controller = TODOBluetoothCtlController::new(opts);

        if synchronous_retry.is_some() {
            eprintln!("[NOTE]: Scanning for bluetooth devices...");
            controller.scan(10)?;
        }

        //let known_devs = scanner.get_known_devices()?;
        eprintln!("[NOTE]: Looking for known bluetooth devices...");
        let devs = controller.list_devices()?;


        // NOTE: this logic could reside in the scanner itself
        if devs.is_empty() && loop_protection < 3 {
            synchronous_retry = Some(SynchronousRescanType::NoneSeen);
            eprintln!("[NOTE]: No Bluetooth devices seen, retrying...");
            continue;
        }

        if should_auto_retry_with_synchronous_scan(opts, &devs, &synchronous_retry) {
            synchronous_retry = Some(SynchronousRescanType::Automatic);
            continue;
        }

        if let Some(prefix) = opts.get_given_device_name_prefix() {
            match get_first_matching_device(&devs, prefix).map(Clone::clone) {
                Ok(dev) => return Ok(dev),
                Err(err) => {
                    if loop_protection < 3 {
                        eprintln!("{}", err);
                        continue;
                    } else {
                        return Err(err)
                    }
                }
            }
        };

        let select_result = devs.select_network(opts);

        if manual_refresh_requested(&select_result) {
            synchronous_retry = Some(SynchronousRescanType::ManuallyRequested);
            continue;
        }

        return select_result;
    }
}

impl Identifiable for BluetoothDevice {
    fn get_public_name(&self) -> &str {
        &self.get_name()
    }
}

impl Selector for Vec<BluetoothDevice> {
    type Item = BluetoothDevice;

    fn get_networks(&self) -> &[BluetoothDevice] {
        self
    }
}
