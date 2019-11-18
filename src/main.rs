// #![deny(warnings)]
use ruwi::run_ruwi;
use std::error::Error;
use std::process::exit;

fn main() {
    let res = run_ruwi();
    match res {
        Ok(int) => int,
        Err(err) => {
            eprintln!("[ERROR]: {}", err.description());
            exit(1);
        }
    };
}
