use crate::interface_management::bring_interface_up;
use crate::run_commands::*;
use crate::structs::*;

static NMCLI_SCAN_ERR_MSG: &str = concat!(
    "Failed to load cached list of seen networks with `nmcli`. Is it installed? ",
    "You can also select a different scanning method with -s (try 'wpa_cli' or 'iw'), ",
    "or you can manually specify an essid with -e.",
);

pub(crate) fn run_nmcli_scan(options: &WifiConnectOptions, scan_type: ScanType) -> Result<ScanResult, RuwiError> {
    bring_interface_up(options)?;
    let scan_output = if options.get_force_synchronous_scan() || options.get_synchronous_retry().is_some() {
        run_nmcli_scan_cmd_synchronous(options)?
    } else {
        run_nmcli_scan_cmd(options)?
    };

    Ok(ScanResult {
        scan_type,
        scan_output,
    })
}

fn run_nmcli_scan_cmd(options: &WifiConnectOptions) -> Result<String, RuwiError> {
    run_command_pass_stdout(
        options.d(),
        "nmcli",
        &["--escape", "no", "--color", "no", "-g", "SECURITY,SIGNAL,SSID", "device", "wifi", "list"],
        RuwiErrorKind::FailedToRunNmcliScan,
        NMCLI_SCAN_ERR_MSG,
    )
}

fn run_nmcli_scan_cmd_synchronous(options: &WifiConnectOptions) -> Result<String, RuwiError> {
    run_command_pass_stdout(
        options.d(),
        "nmcli",
        &["--escape", "no", "--color", "no", "-g", "SECURITY,SIGNAL,SSID", "device", "wifi", "list", "--rescan", "yes"],
        RuwiErrorKind::FailedToRunNmcliScanSynchronous,
        NMCLI_SCAN_ERR_MSG,
    )
}
