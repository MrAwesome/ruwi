use crate::interface_management::bring_interface_up;
use crate::run_commands::run_command;
use crate::structs::*;
use crate::wpa_cli_initialize::initialize_wpa_cli;
use std::thread;
use std::time::Duration;

use std::io;
use std::process::{Command, Stdio};

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
        dbg![&res];
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
        dbg![&output_res];
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

    // TODO: Find if there's a command to wait until the interface is really up
    thread::sleep(Duration::from_secs(1));

    // Trigger a scan. Failure can safely be ignored.
    run_iw_scan_trigger(options).ok();

    let output = run_command(
        options,
        "iw",
        &[&options.interface, "scan", "dump"],
        concat!(
            "Failed to scan with `iw`. Is it installed? ",
            "You can also select a different scanning method with -s (try 'iw' or 'iwlist'), ",
            "or you can manually specify an essid with -e.",
        ),
    )?;

    Ok(ScanResult {
        scan_type: ScanType::IW,
        output,
    })
}

// Initiate a rescan. This command should return instantaneously.
fn run_iw_scan_trigger(options: &Options) -> io::Result<String> {
    run_command(
        options,
        "iw",
        &[&options.interface, "scan", "trigger"],
        "Triggering scan with iw failed. This should be ignored.",
    )
}
