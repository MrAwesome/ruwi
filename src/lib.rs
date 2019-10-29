#![deny(warnings)]
#![deny(clippy::all)]
pub mod connect;
pub mod netctl_config_writer;
pub mod output;
pub mod parse;
pub mod password_prompt;
pub mod scan;
pub mod select;
pub mod select_network;
pub mod sort_networks;
pub mod structs;

use connect::*;
use output::*;
use parse::*;
use password_prompt::*;
use scan::*;
use select_network::*;
use sort_networks::*;
use structs::*;

pub fn run_ruwi(options: &Options) -> RuwiResult {
    // TODO: push the result handling back into the parser? or have an overall error handler
    // TODO: handle all the unwraps
    let scan_result = wifi_scan(&options).unwrap();
    let parse_results = parse_result(&options, &scan_result).unwrap();
    let available_networks = get_and_sort_available_networks(&options, &parse_results);
    let selected_network = select_network(&options, &available_networks).unwrap();
    let encryption_key = get_password(&options, &selected_network).unwrap();
    let output_results = send_outputs(&options, &selected_network, &encryption_key);
    let connection_result = connect_to_network(&options, &selected_network);
    RuwiResult {
        output_results,
        connection_result,
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
