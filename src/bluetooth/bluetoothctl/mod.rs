use crate::run_commands::SystemCommandRunner;

use super::*;

use std::error::Error;
use std::fmt;
use std::fmt::Display;

pub(crate) struct BluetoothCtlController<'a, O: Global> {
    opts: &'a O,
}

impl<'a, O: Global> BluetoothCtlController<'a, O> {
    pub(crate) fn new(opts: &'a O) -> Self {
        Self { opts }
    }
}

impl<'a, O: Global> BluetoothService for BluetoothCtlController<'a, O> {
    type Opts = O;

    fn get_opts(&self) -> &O {
        self.opts
    }
    fn list_devices(&self) -> Result<Vec<BluetoothDevice>, RuwiError> {
        let output = SystemCommandRunner::new(self.get_opts(), "bluetoothctl", &["devices"])
            .run_command_pass_stdout(
                RuwiErrorKind::FailedToFindDevicesWithBluetoothCtl,
                "Failed to list bluetooth devices using bluetoothctl!",
            )?;

        parse_bluetoothctl_devices_output(&output)
    }
    fn power_on(&self) -> Result<(), RuwiError> {
        SystemCommandRunner::new(self.get_opts(), "bluetoothctl", &["power", "on"])
            .run_command_pass(
                RuwiErrorKind::FailedToRunBluetoothCtlPowerOn,
                "Failed to power on bluetooth device using bluetoothctl!",
            )
    }
    fn power_off(&self) -> Result<(), RuwiError> {
        SystemCommandRunner::new(self.get_opts(), "bluetoothctl", &["power", "off"])
            .run_command_pass(
                RuwiErrorKind::FailedToRunBluetoothCtlPowerOff,
                "Failed to power off bluetooth device using bluetoothctl!",
            )
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
}

#[derive(Debug)]
struct ParseError;
impl Error for ParseError {}
impl Display for ParseError {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

fn parse_bluetoothctl_devices_output(output: &str) -> Result<Vec<BluetoothDevice>, RuwiError> {
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

    let dev = BluetoothDevice::builder().name(name).addr(addr).build();

    Ok(dev)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_devices() -> Result<(), RuwiError> {
        let output = include_str!("samples/bluetoothctl_two_devices.txt");
        let devices = parse_bluetoothctl_devices_output(output)?;
        let expected_dev1 = BluetoothDevice::builder()
            .addr("00:0D:44:BE:7D:EB")
            .name("UE Boombox")
            .build();

        let expected_dev2 = BluetoothDevice::builder()
            .addr("04:52:C7:C3:73:11")
            .name("Gleez Head")
            .build();

        assert![devices.contains(&expected_dev1)];
        assert![devices.contains(&expected_dev2)];

        Ok(())
    }

    #[test]
    fn no_devices() -> Result<(), RuwiError> {
        let output = "";
        let devices = parse_bluetoothctl_devices_output(output)?;
        assert![devices.is_empty()];

        Ok(())
    }
}
