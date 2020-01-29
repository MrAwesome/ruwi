mod iw_scan;
use iw_scan::run_iw_scan;

mod nmcli_scan;
use nmcli_scan::run_nmcli_scan;

use crate::rerr;
use crate::run_commands::*;
use crate::structs::*;
use crate::options::interfaces::*;
use crate::service_management::GetService;

use std::fs::File;
use std::io;
use std::io::Read;

pub(crate) static ALLOWED_SYNCHRONOUS_RETRIES: u32 = 15;
pub(crate) static SYNCHRONOUS_RETRY_DELAY_SECS: f64 = 0.3;

pub(crate) static DEVICE_OR_RESOURCE_BUSY_EXIT_CODE: i32 = 240;

pub(crate) fn wifi_scan<O>(options: &O, synchronous_rescan: Option<SynchronousRescanType>) -> Result<ScanResult, RuwiError> where O: Global + Wifi + LinuxNetworkingInterface {
    let sm = options.get_scan_method().clone();
    let st = options.get_scan_type().clone();
    st.get_service().start(options)?;

    let res = match sm {
        ScanMethod::ByRunning => match &st {
            ScanType::Wifi(WifiScanType::Nmcli) => run_nmcli_scan(options, st, synchronous_rescan),
            ScanType::Wifi(WifiScanType::WpaCli) => run_wpa_cli_scan(options, st),
            ScanType::Wifi(WifiScanType::IW) => run_iw_scan(options, st, synchronous_rescan),
            ScanType::Wifi(WifiScanType::RuwiJSON) => 
                Err(rerr!(
                    RuwiErrorKind::InvalidScanTypeAndMethod,
                    "There is currently no binary for providing JSON results, you must format them yourself and pass in via stdin or from a file.",
                ))
        },
        ScanMethod::FromFile(filename) => get_scan_contents_from_file(options, st, &filename),
        ScanMethod::FromStdin => get_scan_contents_from_stdin(options, st),
    };

    if options.d() {
        dbg![&res];
    }
    res
}

fn get_scan_contents_from_stdin<O>(
    _options: &O,
    scan_type: ScanType,
) -> Result<ScanResult, RuwiError> where O: Global {
    let mut stdin_contents = "".into();
    io::stdin().read_to_end(&mut stdin_contents).map_err(|_e| {
        rerr!(
            RuwiErrorKind::FailedToReadScanResultsFromStdin,
            "Failed to get scan results from stdin!"
        )
    })?;

    let scan_output = String::from_utf8_lossy(&stdin_contents).into();

    Ok(ScanResult {
        scan_type,
        scan_output,
    })
}

fn get_scan_contents_from_file<O>(
    _options: &O,
    scan_type: ScanType,
    filename: &str,
) -> Result<ScanResult, RuwiError> where O: Global {
    let file_read_err = |_e: io::Error| {
        rerr!(
            RuwiErrorKind::FailedToReadScanResultsFromFile,
            format!("Failed to read scan contents from `{}`. Does that file exist?", filename)
        )
    };
    let mut file_contents = "".into();
    File::open(filename)
        .map_err(file_read_err)?
        .read_to_end(&mut file_contents)
        .map_err(file_read_err)?;

    let scan_output = String::from_utf8_lossy(&file_contents).into();

    Ok(ScanResult {
        scan_type,
        scan_output,
    })
}

fn run_wpa_cli_scan<O>(options: &O, scan_type: ScanType) -> Result<ScanResult, RuwiError> where O: Global {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::{Output, Command, Stdio};

    pub(crate) static FAKE_OUTPUT: &str = "LOLWUTFAKEIWLOL";
    pub(crate) fn command_fail_with_exitcode(code: i32) -> Output {
        Command::new("/bin/sh")
            .arg("-c")
            .arg(format!("exit {}", code))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap()
    }

    pub(crate) fn command_fail_with_exitcode_1<O>(_options: &O) -> Result<Output, RuwiError> where O: Global {
        Ok(command_fail_with_exitcode(1))
    }

    pub(crate) fn command_fail_with_device_or_resource_busy<O>(_options: &O) -> Result<Output, RuwiError> where O: Global {
        Ok(command_fail_with_exitcode(
            DEVICE_OR_RESOURCE_BUSY_EXIT_CODE,
        ))
    }

    pub(crate) fn command_pass<O>(_opts: &O) -> Result<Output, RuwiError> where O: Global {
        Ok(Command::new("/bin/echo")
            .arg(FAKE_OUTPUT)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap())
    }

    #[test]
    fn test_enough_time_to_retry() {
        let expected_min_secs_needed_to_abort_scan = 4.0;
        assert![
            f64::from(ALLOWED_SYNCHRONOUS_RETRIES) * SYNCHRONOUS_RETRY_DELAY_SECS
                > expected_min_secs_needed_to_abort_scan
        ];
    }
}
