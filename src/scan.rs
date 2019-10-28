use crate::structs::*;

use std::process::{Child, Command, ExitStatus, Output, Stdio};

pub fn wifi_scan(options: Options) -> Result<ScanResult, ScanError> {
    match &options.scan_type {
        ScanType::WpaCli => run_wpa_cli_scan(options),
        ScanType::IW => Err(ScanError::NotImplemented),
        ScanType::IWList => Err(ScanError::NotImplemented),
    }
}

pub fn run_wpa_cli_scan(options: Options) -> Result<ScanResult, ScanError> {
    let res = Command::new("wpa_cli")
        .arg("scan_results")
        // NOTE: this is not required, make interface optional for this command but not for iw
        .arg(options.interface)
        .stdout(Stdio::piped())
        .output()
        // Figure out the exit code of dev/resource busy and handle it appropriately
        .or(Err(ScanError::DeviceOrResourceBusy))?;
    let output = String::from_utf8_lossy(&res.stdout).to_string();
    Ok(ScanResult {
        scan_type: ScanType::WpaCli,
        output: output,
    })
}
