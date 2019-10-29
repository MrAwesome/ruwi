use ruwi::run_ruwi;
use ruwi::structs::*;

fn main() {
    let options = Options {
        scan_type: ScanType::WpaCli,
        selection_method: SelectionMethod::Dmenu,
        output_types: vec![OutputType::NetctlConfig],
        interface: "wlp3s0".to_string(),
        connect_via: Some(ConnectionType::Netctl),
        debug: true,
    };

    run_ruwi(&options);
}
