use crate::interface_management::bring_interface_up;
use crate::rerr;
use crate::run_commands::*;
use crate::structs::*;
use std::process::Output;

#[cfg(not(test))]
use std::thread;
#[cfg(not(test))]
use std::time::Duration;

use crate::wifi_scan::{
    ALLOWED_SYNCHRONOUS_RETRIES, DEVICE_OR_RESOURCE_BUSY_EXIT_CODE, SYNCHRONOUS_RETRY_DELAY_SECS,
};

// TODO: make function, include exact command being run
// TODO: fix this failure showing up in dryruns
static NMCLI_SCAN_ERR_MSG: &str = concat!(
    "Failed to load cached list of seen networks with `nmcli`. Is it installed? ",
    "You can also select a different scanning method with -s (try 'wpa_cli' or 'iw'), ",
    "or you can manually specify an essid with -e.",
);

pub(crate) fn run_nmcli_scan(options: &Options, scan_type: ScanType) -> Result<ScanResult, RuwiError> {
    bring_interface_up(options)?;
    let scan_output = if options.force_synchronous_scan || options.synchronous_retry.is_some() {
        run_nmcli_scan_synchronous(options)?
    } else {
        run_nmcli_scan(options)?
    };

    Ok(ScanResult {
        scan_type,
        scan_output,
    })
}

fn run_nmcli_scan(options: &Options) -> Result<String, RuwiError> {
    run_command_pass_stdout(
        options.debug,
        "nmcli",
        &["--escape", "no", "--color", "no", "-g", "SECURITY,SIGNAL,SSID", "device", "wifi", "list"]
        RuwiErrorKind::FailedToRunNmcliScan,
        NM_ERR_MSG,
    )
}

fn run_nmcli_scan_synchronous(options: &Options) -> Result<String, RuwiError> {
    run_command_pass_stdout(
        options.debug,
        "nmcli",
        &["--escape", "no", "--color", "no", "-g", "SECURITY,SIGNAL,SSID", "device", "wifi", "list", "--rescan", "yes"]
        RuwiErrorKind::FailedToRunIWScanDump,
        IW_SCAN_DUMP_ERR_MSG,
    )
}
