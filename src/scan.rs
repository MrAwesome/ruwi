use crate::errbox;
use crate::interface_management::bring_interface_up;
use crate::run_commands::*;
use crate::structs::*;
use crate::wpa_cli_initialize::initialize_wpa_cli;

use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::Duration;

const ALLOWED_SYNCHRONOUS_RETRIES: u64 = 5;
const SYNCHRONOUS_RETRY_DELAY_SECS: u64 = 1;

// TODO: make function, include exact command being run
const IW_SCAN_DUMP_ERR_MSG: &str = concat!(
    "Failed to load cached list of seen networks with `iw`. Is it installed? ",
    "You can also select a different scanning method with -s (try 'wpa_cli' or 'iwlist'), ",
    "or you can manually specify an essid with -e.",
);

const IW_SCAN_SYNC_ERR_MSG: &str = concat!(
    "Failed to scan with `iw`. Is it installed? ",
    "You can also select a different scanning method with -s (try 'wpa_cli' or 'iwlist'), ",
    "or you can manually specify an essid with -e.",
);

const DEVICE_OR_RESOURCE_BUSY_EXIT_CODE: i32 = 240;

pub(crate) fn wifi_scan(options: Options) -> Result<ScanResult, ErrBox> {
    let res = match &options.scan_type {
        ScanType::WpaCli => run_wpa_cli_scan(&options),
        ScanType::IW => run_iw_scan(&options),
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

fn run_wpa_cli_scan(options: &Options) -> Result<ScanResult, ErrBox> {
    initialize_wpa_cli(options)?;
    // TODO: use new method
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
            scan_output: String::from_utf8_lossy(&o.stdout).to_string(),
        }),
        Err(_e) => Err(errbox!(concat!(
            "Failed to scan with `wpa_cli scan_results`. ",
            "Is wpa_supplicant running? Is it installed? ",
            "You can also select a different scanning method with -s (try 'iw' or 'iwlist'), ",
            "or you can manually specify an essid with -e.",
        ))),
    }
}

fn run_iw_scan(options: &Options) -> Result<ScanResult, ErrBox> {
    bring_interface_up(options)?;
    let mut scan_output = run_iw_scan_dump(options)?;

    if scan_output.is_empty() {
        scan_output = run_iw_scan_synchronous(options)?;
    } else {
        run_iw_scan_trigger(options).ok();
    }

    Ok(ScanResult {
        scan_type: ScanType::IW,
        scan_output,
    })
}

fn run_iw_scan_synchronous(options: &Options) -> Result<String, ErrBox> {
    let mut retries = ALLOWED_SYNCHRONOUS_RETRIES;
    loop {
        let synchronous_run_output = run_iw_scan_synchronous_cmd(options)?;
        if synchronous_run_output.status.code() == Some(DEVICE_OR_RESOURCE_BUSY_EXIT_CODE) {
            retries -= 1;
            if retries > 0 {
                thread::sleep(Duration::from_secs(SYNCHRONOUS_RETRY_DELAY_SECS));
                continue;
            } else {
                return Err(errbox!(IW_SCAN_SYNC_ERR_MSG));
            }
        } else if !synchronous_run_output.status.success() {
            return Err(errbox!(IW_SCAN_SYNC_ERR_MSG));
        } else {
            return Ok(String::from_utf8_lossy(&synchronous_run_output.stdout).to_string());
        }
    }
}

fn run_iw_scan_synchronous_cmd(options: &Options) -> Result<Output, ErrBox> {
    run_command_output(options.debug, "iw", &[&options.interface, "scan"])
}

fn run_iw_scan_dump(options: &Options) -> Result<String, ErrBox> {
    run_command_pass_stdout(
        options.debug,
        "iw",
        &[&options.interface, "scan", "dump"],
        IW_SCAN_DUMP_ERR_MSG,
    )
}

// Initiate a rescan. This command should return instantaneously.
fn run_iw_scan_trigger(options: &Options) -> Result<String, ErrBox> {
    run_command_pass_stdout(
        options.debug,
        "iw",
        &[&options.interface, "scan", "trigger"],
        "Triggering scan with iw failed. This should be ignored.",
    )
}
