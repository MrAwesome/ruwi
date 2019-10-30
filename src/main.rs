// #![deny(warnings)]
use ruwi::cmdline_parser::*;
use ruwi::run_ruwi;

fn main() {
    let options = get_options();

    run_ruwi(&options);
}
