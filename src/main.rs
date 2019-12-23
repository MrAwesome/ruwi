#[cfg(target_os = "linux")]
use ruwi::run_ruwi;
#[cfg(target_os = "linux")]
use std::process::exit;

#[cfg(target_os = "linux")]
fn main() {
    let x = run_ruwi();
    match x {
        Ok(()) => (),
        Err(err) => {
            eprintln!("[ERR]: {}", err);
            // TODO: different error codes for different errors? Default exit code, with the ability to pass in custom codes?
            exit(1);
        }
    };
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("[ERR] Ruwi is currently only supported on Linux. PRs to support other operating systems are happily accepted!");
}
