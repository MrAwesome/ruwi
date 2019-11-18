use crate::interface_management::bring_interface_up;
use crate::structs::*;
use crate::wpa_cli_initialize::initialize_wpa_cli;
use std::thread;
use std::time::Duration;

use std::io;
use std::process::{Command, Output, Stdio};

pub(crate) fn wifi_scan(options: &Options) -> io::Result<ScanResult> {
    let res = match &options.scan_type {
        ScanType::WpaCli => run_wpa_cli_scan(options),
        ScanType::IW => run_iw_scan(options),
        x @ ScanType::IWList => Err(nie(x)),
        // TODO: Add nmcli scan
        // nmcli device wifi rescan
        // nmcli device wifi list
    };

    if options.debug {
        dbg!(&res);
    }

    res
}

fn run_wpa_cli_scan(options: &Options) -> io::Result<ScanResult> {
    initialize_wpa_cli(options)?;
    let output_res = Command::new("wpa_cli")
        .arg("scan_results")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output();

    if options.debug {
        dbg!(&output_res);
    }

    match &output_res {
        Ok(o) => Ok(ScanResult {
            scan_type: ScanType::WpaCli,
            output: String::from_utf8_lossy(&o.stdout).to_string(),
        }),
        Err(_e) => Err(io::Error::new(
            io::ErrorKind::Other,
            concat!(
                "Failed to scan with `wpa_cli scan_results`. ",
                "Is wpa_supplicant running? Is it installed? ",
                "You can also select a different scanning method with -s (try 'iw' or 'iwlist'), ",
                "or you can manually specify an essid with -e.",
            ),
        )),
    }
}

fn run_iw_scan(options: &Options) -> io::Result<ScanResult> {
    bring_interface_up(options)?;

    // TODO: is there a way to avoid this?
    thread::sleep(Duration::from_secs(1));

    run_iw_scan_trigger(options);

    let output_res = Command::new("iw")
        .arg(&options.interface)
        .arg("scan")
        .arg("dump")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output();

    if options.debug {
        dbg!(&output_res);
    }

    match &output_res {
        Ok(o) => {
            let output = String::from_utf8_lossy(&o.stdout).to_string();
            Ok(ScanResult {
                scan_type: ScanType::IW,
                output,
            })
        }
        Err(_e) => Err(io::Error::new(
            io::ErrorKind::Other,
            concat!(
                "Failed to scan with `iw`. Is it installed? ",
                "You can also select a different scanning method with -s (try 'iw' or 'iwlist'), ",
                "or you can manually specify an essid with -e.",
            ),
        )),
    }
}

// Initiate a rescan. Failure is ignored. This command should return instantaneously.
fn run_iw_scan_trigger(options: &Options) -> io::Result<Output> {
    let spawn_res = Command::new("iw")
        .arg(&options.interface)
        .arg("scan")
        .arg("trigger")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    if options.debug {
        dbg!(&spawn_res);
    }

    let cmd_res = spawn_res?.wait_with_output();

    if options.debug {
        dbg!(&cmd_res);
    }
    Ok(cmd_res?)
}
