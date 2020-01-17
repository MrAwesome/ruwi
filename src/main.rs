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
            // TODO: Different error codes for different errors? Default exit code, with the ability to pass in custom codes?
            // TODO: Ability to print stdout/stderr of failing commands?
            exit(1);
        }
    };
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("[ERR] Ruwi is currently only supported on Linux. PRs to support other operating systems are happily accepted!");
}
