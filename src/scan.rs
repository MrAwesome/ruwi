use crate::interface_management::bring_interface_up;
use crate::rerr;
use crate::run_commands::*;
use crate::structs::*;
use crate::wpa_cli_initialize::initialize_wpa_cli;

use std::process::Output;
use std::thread;
use std::time::Duration;

static ALLOWED_SYNCHRONOUS_RETRIES: u64 = 10;
static SYNCHRONOUS_RETRY_DELAY_SECS: f64 = 0.3;

// TODO: make function, include exact command being run
static IW_SCAN_DUMP_ERR_MSG: &str = concat!(
    "Failed to load cached list of seen networks with `iw`. Is it installed? ",
    "You can also select a different scanning method with -s (try 'wpa_cli' or 'iwlist'), ",
    "or you can manually specify an essid with -e.",
);

static IW_SCAN_SYNC_ERR_MSG: &str = concat!(
    "Failed to scan with `iw`. Is it installed? ",
    "You can also select a different scanning method with -s (try 'wpa_cli' or 'iwlist'), ",
    "or you can manually specify an essid with -e.",
);

const DEVICE_OR_RESOURCE_BUSY_EXIT_CODE: i32 = 240;

pub(crate) fn wifi_scan(options: Options) -> Result<ScanResult, RuwiError> {
    let res = match &options.scan_type {
        ScanType::WpaCli => run_wpa_cli_scan(&options),
        ScanType::IW => run_iw_scan(&options),
        x @ ScanType::IWList => Err(nie(x)),
        // TODO: Add nmcli scan
        // nmcli device wifi rescan
        // nmcli device wifi list
    };

    let todo = "for iwlist, you can scan with scan and dump with scan_last";

    if options.debug {
        dbg![&res];
    }
    res
}

fn run_wpa_cli_scan(options: &Options) -> Result<ScanResult, RuwiError> {
    initialize_wpa_cli(options)?;

    let err_msg = concat!(
        "Failed to scan with `wpa_cli scan_results`. ",
        "Is wpa_supplicant running? Is it installed? ",
        "You can also select a different scanning method with -s (try 'iw' or 'iwlist'), ",
        "or you can manually specify an essid with -e.",
    );

    // TODO: add scan_results latest
    let scan_output = run_command_pass_stdout(
        options.debug,
        "wpa_cli",
        &["scan_results"],
        RuwiErrorKind::FailedToScanWithWPACli,
        err_msg,
    )?;

    if options.debug {
        dbg![&scan_output];
    }
    Ok(ScanResult {
        scan_type: ScanType::WpaCli,
        scan_output,
    })
}

// TODO: unit test
fn run_iw_scan(options: &Options) -> Result<ScanResult, RuwiError> {
    bring_interface_up(options)?;
    let scan_output = if options.force_synchronous_scan {
        run_iw_scan_synchronous(options)?
    } else {
        let mut scan_output = run_iw_scan_dump(options)?;
        if scan_output.is_empty() {
            scan_output = run_iw_scan_synchronous(options)?;
        } else {
            run_iw_scan_trigger(options).ok();
        }
        scan_output
    };

    Ok(ScanResult {
        scan_type: ScanType::IW,
        scan_output,
    })
}

// TODO: unit test
fn run_iw_scan_synchronous(options: &Options) -> Result<String, RuwiError> {
    let mut retries = ALLOWED_SYNCHRONOUS_RETRIES;
    abort_ongoing_iw_scan(options).ok();
    loop {
        let synchronous_run_output = run_iw_scan_synchronous_cmd(options)?;
        if synchronous_run_output.status.code() == Some(DEVICE_OR_RESOURCE_BUSY_EXIT_CODE) {
            retries -= 1;
            if retries > 0 {
                thread::sleep(Duration::from_secs_f64(SYNCHRONOUS_RETRY_DELAY_SECS));
                continue;
            } else {
                return Err(rerr!(
                    RuwiErrorKind::IWSynchronousScanRanOutOfRetries,
                    IW_SCAN_SYNC_ERR_MSG
                ));
            }
        } else if !synchronous_run_output.status.success() {
            return Err(rerr!(
                RuwiErrorKind::IWSynchronousScanFailed,
                IW_SCAN_SYNC_ERR_MSG
            ));
        } else {
            return Ok(String::from_utf8_lossy(&synchronous_run_output.stdout).to_string());
        }
    }
}

fn run_iw_scan_synchronous_cmd(options: &Options) -> Result<Output, RuwiError> {
    run_command_output(options.debug, "iw", &[&options.interface, "scan"])
}

fn run_iw_scan_dump(options: &Options) -> Result<String, RuwiError> {
    run_command_pass_stdout(
        options.debug,
        "iw",
        &[&options.interface, "scan", "dump"],
        RuwiErrorKind::FailedToRunIWScanDump,
        IW_SCAN_DUMP_ERR_MSG,
    )
}

fn run_iw_scan_trigger(options: &Options) -> Result<String, RuwiError> {
    // Initiate a rescan. This command should return instantaneously.
    run_command_pass_stdout(
        options.debug,
        "iw",
        &[&options.interface, "scan", "trigger"],
        RuwiErrorKind::FailedToRunIWScanTrigger,
        "Triggering scan with iw failed. This should be ignored.",
    )
}

fn abort_ongoing_iw_scan(options: &Options) -> Result<String, RuwiError> {
    run_command_pass_stdout(
        options.debug,
        "iw",
        &[&options.interface, "scan", "abort"],
        RuwiErrorKind::FailedToRunIWScanAbort,
        "Aborting iw scan iw failed. This should be ignored.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enough_time_to_retry() {
        let expected_min_secs_needed_to_abort_scan = 2.0;
        assert![
            ALLOWED_SYNCHRONOUS_RETRIES as f64 * SYNCHRONOUS_RETRY_DELAY_SECS
                > expected_min_secs_needed_to_abort_scan
        ];
    }
}
