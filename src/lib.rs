// Prerequisite packages:
// iw
// wpa_supplicant?

// #![deny(warnings)]
// #![deny(clippy::all)]

extern crate clap;
extern crate regex;
extern crate strum;
extern crate strum_macros;

pub mod cmdline_parser;
pub mod connect;
pub mod get_default_interface;
pub mod initialize;
pub mod interface_management;
pub mod netctl_config_writer;
pub mod output;
pub mod parse;
pub mod password_prompt;
pub mod scan;
pub mod select;
pub mod select_network;
pub mod sort_networks;
pub mod structs;
pub mod strum_utils;

use connect::*;
use initialize::*;
use output::*;
use parse::*;
use password_prompt::*;
use scan::*;
use select_network::*;
use sort_networks::*;
use structs::*;

use std::io;

// TODO(wishlist): macro for `if options.debug { dbg!(arg); } arg`
// TODO: come up with subcommands which only run specified pieces
// TODO: make sure fzf and dmenu are listed as dependencies

pub fn run_ruwi(options: &Options) -> io::Result<RuwiResult> {
    // TODO: bring up interface for scan if needed, take down before netctl switch-to
    // TODO: if there are multiple interfaces seen by 'iw dev', bring up selection, otherwise pick
    // the default
    let _initialize_result = initialize(&options);
    let scan_result = wifi_scan(&options)?;
    let parse_results = parse_result(&options, &scan_result)?;
    let available_networks = get_and_sort_available_networks(&options, &parse_results);
    let selected_network = select_network(&options, &available_networks)?;
    let encryption_key = get_password(&options, &selected_network)?;
    let output_result = send_output(&options, &selected_network, &encryption_key)?;
    let connection_result = connect_to_network(&options, &selected_network)?;
    Ok(RuwiResult {
        output_result,
        connection_result,
    })
}

#[cfg(test)]
mod tests {
    // use super::*;
}
