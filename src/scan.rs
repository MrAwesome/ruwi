use crate::structs::*;

use std::io;
use std::process::{exit, Command, Stdio};

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
pub fn run_wpa_cli_scan(options: &Options) -> io::Result<ScanResult> {
    let output_res = Command::new("wpa_cli")
        .arg("scan_results")
        .stdout(Stdio::piped())
        .output();

    // TODO: CLEANUP

    if options.debug {
        dbg!(&output_res);
    }

    let output = match &output_res {
        Ok(o) => o,
        Err(_e) => {
            eprintln!("Failed to scan with wpa_cli. Is it installed?");
            exit(1);
        }
    };

    match output.status.code() {
        Some(num) => match num {
            2 | 127 => {
                eprintln!(concat!(
                    "`wpa_cli` is not available for scanning interfaces.",
                    "Please install it, select a different scanning method with -s,",
                    "or manually specify a network with -n.",
                ));
                exit(1);
            }
            _ => "",
        },
        None => {
            eprintln!("Failed to scan with wpa_cli. Is it installed?");
            exit(1);
        }
    };

    let output = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(ScanResult {
        scan_type: ScanType::WpaCli,
        output,
    })
}

fn generic_wpa_cli_scan_failure() -> ! {
    eprintln!(concat!("Failed to scan with wpa_cli. Is it installed? ",
                    "Please install it and ensure it's in your $PATH, select a different scanning method with -s,",
                    "or manually specify a network with -n.",
    ));
    exit(1);
}
