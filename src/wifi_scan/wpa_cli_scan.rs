use crate::enums::*;
use crate::errors::*;
use crate::options::interfaces::*;
use crate::run_commands::*;
use crate::structs::*;

pub(crate) fn run_wpa_cli_scan<O>(options: &O, scan_type: ScanType) -> Result<ScanResult, RuwiError>
where
    O: Global,
{
    let err_msg = concat!(
        "Failed to scan with `wpa_cli scan_results`. ",
        "Is wpa_supplicant running? Is it installed? ",
        "You can also select a different scanning method with -s (try 'iw' or 'iwlist'), ",
        "or you can manually specify an essid with -e.",
    );

    // TODO: add scan_results latest
    let scan_output = run_command_pass_stdout(
        options,
        "wpa_cli",
        &["scan_results"],
        RuwiErrorKind::FailedToScanWithWPACli,
        err_msg,
    )?;

    if options.d() {
        dbg![&scan_output];
    }
    Ok(ScanResult {
        scan_type,
        scan_output,
    })
}
