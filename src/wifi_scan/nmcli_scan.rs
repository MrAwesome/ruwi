use crate::interface_management::ip_interfaces::*;
use crate::enums::*;
use crate::errors::*;
use crate::options::interfaces::*;
use crate::run_commands::SystemCommandRunner;
use crate::structs::*;

const NMCLI_SCAN_ERR_MSG: &str = concat!(
    "Failed to load cached list of seen networks with `nmcli`. Is it installed? ",
    "You can also select a different scanning method with wifi -s (try `wpa_cli` or `iw`), ",
    "or you can manually specify an essid with wifi -e.",
);

pub(crate) fn run_nmcli_scan<O>(
    options: &O,
    interface: &WifiIPInterface,
    wifi_scan_type: WifiScanType,
    synchronous_rescan: &Option<SynchronousRescanType>,
) -> Result<ScanResult, RuwiError>
where
    O: Global + Wifi,
{
    interface.bring_up(options)?;
    let scan_output = if options.get_force_synchronous_scan() || synchronous_rescan.is_some() {
        run_nmcli_scan_cmd_synchronous(options)?
    } else {
        run_nmcli_scan_cmd(options)?
    };

    Ok(ScanResult {
        scan_type: ScanType::Wifi(wifi_scan_type),
        scan_output,
    })
}

fn run_nmcli_scan_cmd<O>(options: &O) -> Result<String, RuwiError>
where
    O: Global,
{
    SystemCommandRunner::new(
        options,
        "nmcli",
        &[
            "--escape",
            "no",
            "--color",
            "no",
            "-g",
            "SECURITY,SIGNAL,SSID",
            "device",
            "wifi",
            "list",
        ],
    ).run_command_pass_stdout(
        RuwiErrorKind::FailedToRunNmcliScan,
        NMCLI_SCAN_ERR_MSG,
    )
}

fn run_nmcli_scan_cmd_synchronous<O>(options: &O) -> Result<String, RuwiError>
where
    O: Global,
{
    SystemCommandRunner::new(
        options,
        "nmcli",
        &[
            "--escape",
            "no",
            "--color",
            "no",
            "-g",
            "SECURITY,SIGNAL,SSID",
            "device",
            "wifi",
            "list",
            "--rescan",
            "yes",
        ],
    ).run_command_pass_stdout(
        RuwiErrorKind::FailedToRunNmcliScanSynchronous,
        NMCLI_SCAN_ERR_MSG,
    )
}
