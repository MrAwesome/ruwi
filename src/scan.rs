use crate::interface_management::bring_interface_up;
use crate::structs::*;
use std::thread;
use std::time::Duration;

use std::io;
use std::process::{Command, Stdio};

pub(crate) fn wifi_scan(options: &Options) -> io::Result<ScanResult> {
    let res = match &options.scan_type {
        ScanType::WpaCli => run_wpa_cli_scan(options),
        ScanType::IW => run_iw_scan(options),
        x @ ScanType::IWList => Err(nie(x)),
    };

    if options.debug {
        dbg!(&res);
    }

    res
}

// NOTE: interface is ignored for this command
fn run_wpa_cli_scan(options: &Options) -> io::Result<ScanResult> {
    let output_res = Command::new("wpa_cli")
        .arg("scan_results")
        .stdout(Stdio::piped())
        .output();

    if options.debug {
        dbg!(&output_res);
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
                "You can also select a different scanning method with -s (try 'iw' or 'iwlist'),",
                "or you can manually specify an essid with -e.",
            ),
        )),
    }
}

// TODO: [] do iw scan dump if results are already available
//       [] add a flag for 'force refresh'
fn run_iw_scan(options: &Options) -> io::Result<ScanResult> {
    bring_interface_up(options)?;
    thread::sleep(Duration::from_secs(1));
    let output_res = Command::new("iw")
        .arg(&options.interface)
        .arg("scan")
        .stdout(Stdio::piped())
        .output();

    if options.debug {
        dbg!(&output_res);
    }

    match &output_res {
        Ok(o) => {
            let output = String::from_utf8_lossy(&o.stdout).to_string();
            dbg!(&output);
            Ok(ScanResult {
                scan_type: ScanType::IW,
                output,
            })
        }
        Err(_e) => Err(io::Error::new(
            io::ErrorKind::Other,
            concat!(
                "Failed to scan with `iw`. Is it installed?",
                "You can also select a different scanning method with -s (try 'iw' or 'iwlist'),",
                "or you can manually specify an essid with -e.",
            ),
        )),
    }
}
