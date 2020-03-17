#[cfg(target_os = "linux")]
use ruwi::run_ruwi_cli;
#[cfg(target_os = "linux")]
use std::process::exit;

#[cfg(target_os = "linux")]
fn main() {
    let x = run_ruwi_cli();
    match x {
        Ok(()) => (),
        Err(err) => {
            eprintln!("[ERR]: Run failed! ({:?})", err.kind);
            eprintln!("[ERR]: {}", err);
            if let Some(extra_data) = err.extra_data {
                for (key, val) in &extra_data {
                    eprintln!("* {}: {}", key, val);
                }
            }
            // TODO: Different error codes for different errors? Default exit code, with the ability to pass in custom codes?
            exit(1);
        }
    };
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("[ERR] Ruwi is currently only supported on Linux. PRs to support other operating systems are happily accepted!");
}
