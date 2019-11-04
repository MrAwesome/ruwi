// #![deny(warnings)]
use ruwi::cmdline_parser::*;
use ruwi::run_ruwi;
use std::error::Error;
use std::process::exit;

fn main() {
    // TODO: one more layer of results passing
    let options = get_options().unwrap();

    let res = run_ruwi(&options);
    match res {
        Ok(int) => int,
        Err(err) => {
            eprintln!("{}", err.description());
            exit(1);
        }
    };
}
