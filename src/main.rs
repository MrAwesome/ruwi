#[cfg(target_os = "linux")]
use ruwi::run_ruwi_cli;

#[cfg(target_os = "linux")]
fn main() {
    let run_result = run_ruwi_cli();
    if let Err(err) = run_result {
        err.print_error();
        let exit_code = err.get_linux_exit_code();
        std::process::exit(exit_code);
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("[ERR]: Ruwi is currently only supported on Linux. PRs to support other operating systems are happily accepted!");
}
