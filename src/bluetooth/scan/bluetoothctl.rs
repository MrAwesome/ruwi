use std::error::Error;
use std::fmt;
use std::fmt::Display;

use crate::prelude::*;
use crate::run_commands::SystemCommandRunner;

use super::BluetoothDeviceScanner;
use super::super::BluetoothDevice;

pub(crate) struct BluetoothCtlDeviceScanner<'a, O: Global> {
    opts: &'a O,
}

impl<'a, O: Global> BluetoothCtlDeviceScanner<'a, O> {
    pub(crate) fn new(opts: &'a O) -> Self {
       Self {
           opts
       }
    }
}

impl<'a, O: Global> BluetoothDeviceScanner<O> for BluetoothCtlDeviceScanner<'a, O> {
    fn get_opts(&self) -> &O {
        self.opts
    }

    fn scan(&self, scan_secs: usize) -> Result<(), RuwiError> {
        SystemCommandRunner::new(
            self.get_opts(),
            "bluetoothctl",
            &["--timeout", &scan_secs.to_string(), "scan", "on"],
        )
        .run_command_pass(
            RuwiErrorKind::FailedToScanWithBluetoothCtl,
            "Failed to scan for bluetooth devices using bluetoothctl! Are you running as root?",
        )
    }
    fn get_devices(&self) -> Result<Vec<BluetoothDevice>, RuwiError> {
        let output = SystemCommandRunner::new(
            self.get_opts(),
            "bluetoothctl",
            &["devices"],
        )
        .run_command_pass_stdout(
            RuwiErrorKind::FailedToFindDevicesWithBluetoothCtl,
            "Failed to list bluetooth devices using bluetoothctl!",
        )?;

        parse_bluetoothctl_devices_output(output)
    }
}

#[derive(Debug)]
struct ParseError;
impl Error for ParseError {}
impl Display for ParseError {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

fn parse_bluetoothctl_devices_output(output: String) -> Result<Vec<BluetoothDevice>, RuwiError> {
    let mut devices = vec![];
    for line in output.lines() {
        let res = parse_bluetoothctl_devices_line(line);
        match res {
            Ok(dev) => devices.push(dev),
            Err(_) => eprintln!("Failed to parse `bluetoothctl devices` line: {}", line),
        }
    }
    Ok(devices)
}

fn parse_bluetoothctl_devices_line(line: &str) -> Result<BluetoothDevice, ParseError> {
    let mut tokens = line.split_ascii_whitespace();

    // Skip first field
    tokens.next();

    let addr = tokens.next().ok_or(ParseError)?.to_string();

    // Note: this will not work correctly for devices with multiple consecutive spaces in the name
    let name_tokens = tokens.collect::<Vec<&str>>();
    if name_tokens.is_empty() {
        return Err(ParseError);
    }
    let escaped_name = name_tokens.join(" ");
    let name = escaped_name;

    let dev = BluetoothDevice::builder()
        .name(name)
        .addr(addr)
        .build();

    Ok(dev)
}
