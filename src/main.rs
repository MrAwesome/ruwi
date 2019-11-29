// #![deny(warnings)]
use ruwi::run_ruwi;

fn main() {
    let x = run_ruwi();
    match x {
        Ok(()) => (),
        Err(err) => eprintln!("[ERR]: {}", err),
    };
}
