use crate::structs::*;

use std::io;
use std::process::{Command, Stdio};

pub fn wifi_scan(options: &Options) -> io::Result<ScanResult> {
    let res = match &options.scan_type {
        ScanType::WpaCli => run_wpa_cli_scan(options),
        x @ ScanType::IW => Err(nie(x)),
        x @ ScanType::IWList => Err(nie(x)),
    };

    if options.debug {
        dbg!(&res);
    }

    res
}

// NOTE: interface is ignored for this command
pub fn run_wpa_cli_scan(_options: &Options) -> io::Result<ScanResult> {
    let res = Command::new("wpa_cli")
        .arg("scan_results")
        .stdout(Stdio::piped())
        .output()
        // TODO: Figure out the exit code of dev/resource busy and handle it appropriately
        ?;
    // TODO: check for exit status and return scanerror if nonzero
    let output = String::from_utf8_lossy(&res.stdout).to_string();
    Ok(ScanResult {
        scan_type: ScanType::WpaCli,
        output,
    })
}
