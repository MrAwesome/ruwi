use ruwi::connect::*;
use ruwi::output::*;
use ruwi::parse::*;
use ruwi::password_prompt::*;
use ruwi::scan::*;
use ruwi::select_network::*;
use ruwi::structs::*;

// TODO: During debugging only
#[allow(unused_must_use)]
fn main() {
    let options = Options {
        scan_type: ScanType::WpaCli,
        selection_method: SelectionMethod::Dmenu,
        output_types: vec![OutputType::NetctlConfig],
        interface: "wlp3s0".to_string(),
        connect_via: Some(ConnectionType::Netctl),
    };

    // TODO: handle instead of unwrap
    let scan_result = wifi_scan(&options).unwrap();
    // TODO: push the result handling back into the parser? or have an overall error handler
    // which prints diagnostics when fatal errors are encountered
    let parse_results = parse_result(&options, &scan_result);
    let mut available_networks = parse_results
        .clone()
        .expect("Failed to parse!")
        .seen_networks;
    // We want the strongest networks first
    available_networks.sort();
    available_networks.reverse();
    let rev_sorted_available_networks = available_networks;
    let selected_network = select_network(&options, &rev_sorted_available_networks);

    // TODO: handle intelligently:
    let selected_network = selected_network.unwrap();
    let encryption_key = get_password(&options, &selected_network);

    // TODO: handle intelligently:
    let encryption_key = encryption_key.unwrap();
    let output_results = send_outputs(&options, &selected_network, &encryption_key);
    let connection_result = connect_to_network(&options, &selected_network);
    dbg!(scan_result);
    dbg!(parse_results);
    dbg!(rev_sorted_available_networks);
    dbg!(selected_network);
    dbg!(output_results);
    dbg!(connection_result);
}
