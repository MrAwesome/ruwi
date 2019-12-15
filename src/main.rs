// #![deny(warnings)]
use ruwi::run_ruwi;
use std::process::exit;

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
