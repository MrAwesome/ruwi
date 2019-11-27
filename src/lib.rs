// Prerequisite packages:
// iw
// wpa_supplicant?

// #![deny(warnings)]
// #![deny(clippy::all)]

extern crate clap;
extern crate regex;
extern crate strum;
extern crate strum_macros;

pub(crate) mod annotate_networks;
pub(crate) mod cmdline_parser;
pub(crate) mod connect;
pub(crate) mod find_known_network_names;
pub(crate) mod get_default_interface;
pub(crate) mod interface_management;
pub(crate) mod netctl_config_writer;
pub(crate) mod output;
pub(crate) mod parse;
pub(crate) mod password_prompt;
pub(crate) mod run_commands;
pub(crate) mod scan;
pub(crate) mod select;
pub(crate) mod select_network;
pub(crate) mod sort_networks;
pub(crate) mod structs;
pub(crate) mod strum_utils;
pub(crate) mod wpa_cli_initialize;

use annotate_networks::*;
use cmdline_parser::*;
use connect::*;
use find_known_network_names::*;
use output::*;
use parse::*;
use password_prompt::*;
use scan::*;
use select_network::*;
use sort_networks::*;
use structs::*;

use std::io;

// TODO(high): figure out how to unit test / mock command calls
// TODO(high): add integration tests
// TODO(high): find existing ESSIDs, when a scan turns up known network mark its WirelessNetwork in scan (aka make connecting to known networks easier)
// TODO(high): find a way to unit test without actually running commands. maybe with cfg(test)?
// TODO(high): if networkmanager is used, start it up before going - same with netctl. possibly also stop
// TODO(think): come up with subcommands which only run specified pieces, or at least decide on the functionality this command should have
// TODO(mid): add colors to output / use a real logging library
// TODO(mid): if no networks are seen by `iw dump`, go ahead and just run scan? may need to dump before trigger then
// the other, to prevent cross-contamination
// TODO(mid): figure out if networkmanager connection add with wifi password works - looks like not, just fail if output networkmanager is chosen without connection (or combine output and connection as a single concept, and have "print" as one)
// TODO(wishlist): if there are multiple interfaces seen by 'iw dev', bring up selection, otherwise pick the default
// TODO(wishlist): find a generalized way to do x notifications, for dmenu mode, use to surface failures
// TODO(wishlist): determine whether to use dmenu/fzf/etc based on terminal/X
// TODO(wishlist): allow for using only iw to connect? would encryption keys need to be stored anywhere?
// TODO(later): make sure fzf and dmenu are listed as dependencies
// TODO(think): instead of functions which take options, make a big struct/impl? maybe more than one?
// TODO(think): consider just supporting netctl for now?

pub fn run_ruwi() -> io::Result<()> {
    let options = &get_options()?;
    let selected_network = get_selected_network(options)?;
    let encryption_key = get_password(options, &selected_network)?;
    let _output_result = send_output(options, &selected_network, &encryption_key)?;
    let _connection_result = connect_to_network(options, &selected_network)?;
    Ok(())
    //    Ok(RuwiResult {
    //        output_result,
    //        connection_result,
    //    })
}

pub fn get_selected_network(options: &Options) -> io::Result<WirelessNetwork> {
    if let Some(essid) = &options.given_essid {
        Ok(WirelessNetwork {
            essid: essid.clone(),
            known: false,
            is_encrypted: options.given_password.is_some(),
            bssid: None,
            signal_strength: None,
            channel_utilisation: None,
        })
    } else {
        // TODO: do scan and find in parallel
        let scan_result = wifi_scan(options)?;
        let known_network_names = find_known_network_names(options)?;

        let parse_results = parse_result(options, &scan_result)?;
        let annotated_networks =
            annotate_networks(options, &parse_results.seen_networks, &known_network_names);
        let sorted_networks = sort_available_networks(options, annotated_networks);
        let selected_network = select_network(options, &sorted_networks)?;
        Ok(selected_network)
    }
}
#[cfg(test)]
mod tests {
    // use super::*;
}
