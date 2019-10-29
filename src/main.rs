use ruwi::connect::*;
use ruwi::output::*;
use ruwi::parse::*;
use ruwi::password_prompt::*;
use ruwi::scan::*;
use ruwi::select_network::*;
use ruwi::sort_networks::*;
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

    // TODO: push the result handling back into the parser? or have an overall error handler
    // TODO: handle all the unwraps
    let scan_result = wifi_scan(&options).unwrap();
    let parse_results = parse_result(&options, &scan_result).unwrap();
    let available_networks = get_and_sort_available_networks(&options, &parse_results);
    let selected_network = select_network(&options, &available_networks).unwrap();
    let encryption_key = get_password(&options, &selected_network).unwrap();
    let _output_results = send_outputs(&options, &selected_network, &encryption_key);
    let _connection_result = connect_to_network(&options, &selected_network);
}
