// Prerequisite packages:
// iw
// wpa_supplicant?

// #![deny(warnings)]
// #![deny(clippy::all)]

extern crate clap;
//extern crate fern;
//extern crate log;
extern crate regex;
extern crate strum;
extern crate strum_macros;

pub(crate) mod annotate_networks;
pub(crate) mod cmdline_parser;
pub(crate) mod connect;
pub(crate) mod display_network_for_selection;
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
#[macro_use]
pub(crate) mod macros;
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
use std::thread;
use structs::*;

// TODO(high): figure out how to unit test / mock command calls
// TODO(high): find a way to unit test without actually running commands. maybe with cfg(test)?
// TODO(high): if networkmanager is used, start it up before going - same with netctl. possibly also stop
// TODO(think): come up with subcommands which only run specified pieces, or at least decide on the functionality this command should have
// TODO(mid): add colors to output / use a real logging library
// TODO(mid): go back to using enums for errors, with a mapping to error message to print (enums are MUCH faster than boxing errors)
// TODO(mid): if no networks are seen by `iw dump`, go ahead and just run scan? may need to dump before trigger then
// the other, to prevent cross-contamination
// TODO(mid): figure out if networkmanager connection add with wifi password works - looks like not, just fail if output networkmanager is chosen without connection (or combine output and connection as a single concept, and have "print" as one)
// TODO(mid): add integration test, which takes -e and -p, doesn't try to connect, make sure it creates a netctl config?
// TODO(low): kill wpa_supplicant if trying to use raw iw or networkmanager
// TODO(wishlist): if there are multiple interfaces seen by 'iw dev', bring up selection, otherwise pick the default
// TODO(wishlist): json scan output mode
// TODO(wishlist): find a generalized way to do x notifications, for dmenu mode, use to surface failures
// TODO(wishlist): determine whether to use dmenu/fzf/etc based on terminal/X
// TODO(wishlist): connection/scan type: wicd-cli
// TODO(wishlist): connection/scan type: iw raw (would need to store encryption keys, and at that point ruwi is a connection manager)
// TODO(later): make sure fzf and dmenu are listed as dependencies
// TODO(think): instead of functions which take options, make a big struct/impl? maybe more than one?
// TODO(think): add a -w/--wait or --verify or something to attempt to connect to google/etc?
// TODO(think): consider just supporting netctl for now?
// TODO(think): make -a the default?

pub fn run_ruwi() -> Result<(), ErrBox> {
    let options = &get_options()?;
    let selected_network = get_selected_network(options)?;
    if !selected_network.known {
        let encryption_key = get_password(options, &selected_network)?;
        // TODO: still do output for types which aren't config-based (like "print selected network")?
        let _output_result = send_output(options, &selected_network, &encryption_key)?;
    }

    let _connection_result = connect_to_network(options, &selected_network)?;
    Ok(())
}

pub fn get_selected_network(options: &Options) -> Result<AnnotatedWirelessNetwork, ErrBox> {
    if let Some(essid) = &options.given_essid {
        // TODO: make sure known: true is the right option here, or if more control flow is needed
        //       known: true might create issues with creating netctl configs - walk through it on
        //       paper to find out
        //  TODO: decide if this functionality is worth supporting.
        let is_known = true; // ????
        let is_encrypted = options.given_password.is_some();
        Ok(AnnotatedWirelessNetwork::from_essid(
            essid.clone(),
            is_known,
            is_encrypted,
        ))
    } else {
        let annotated_networks = scan_parse_and_annotate_networks_with_retry(options)?;
        let sorted_networks = sort_and_filter_networks(options, annotated_networks);
        let selected_network = select_network(options, &sorted_networks)?;
        Ok(selected_network)
    }
}

// In automatic mode, we want to make a strong attempt to prioritize known networks.
// Cached results are often out of date, so if no known networks are seen, trust our
// user that they should be, and re-run the scan in synchronous mode.
// This logic lives so high in the stack because known status isn't available
// until we've found the known network names and used them to annotate the seen networks.
fn scan_parse_and_annotate_networks_with_retry(
    options: &Options,
) -> Result<AnnotatedNetworks, ErrBox> {
    let annotated_networks_first_attempt = scan_parse_and_annotate_networks(options)?;

    Ok(
        if should_retry_with_synchronous_scan(options, &annotated_networks_first_attempt) {
            eprintln!("[NOTE]: No known networks seen in auto mode using cached scan results. Manually scanning now. ");
            scan_parse_and_annotate_networks(&options.with_synchronous_scan())?
        } else {
            annotated_networks_first_attempt
        },
    )
}

fn should_retry_with_synchronous_scan(
    opts: &Options,
    annotated_networks: &AnnotatedNetworks,
) -> bool {
    // TODO: possibly just rerun everything in synchronous mode if any problems are encountered?
    let is_an_auto_mode = opts.auto_mode == AutoMode::Auto || opts.auto_mode == AutoMode::AutoNoAsk;
    let known_network_found = annotated_networks.networks.iter().any(|x| x.known);
    let no_known_networks_found_in_auto_modes = is_an_auto_mode && !known_network_found;

    let conditions = vec![no_known_networks_found_in_auto_modes];
    conditions.iter().any(|&x| x)
}

fn scan_parse_and_annotate_networks(options: &Options) -> Result<AnnotatedNetworks, ErrBox> {
    let (known_network_names, scan_result) = gather_data(options)?;
    let parse_results = parse_result(options, &scan_result)?;
    let annotated_networks =
        annotate_networks(options, &parse_results.seen_networks, &known_network_names);
    Ok(annotated_networks)
}

fn gather_data(options: &Options) -> Result<(KnownNetworks, ScanResult), ErrBox> {
    let (opt1, opt2) = (options.clone(), options.clone());
    let get_nw_names = thread::spawn(|| find_known_network_names(opt1));
    let get_scan_results = thread::spawn(|| wifi_scan(opt2));

    let known_network_names = await_thread(get_nw_names)??;
    let scan_result = await_thread(get_scan_results)??;

    Ok((known_network_names, scan_result))
}

#[inline]
fn await_thread<T>(handle: thread::JoinHandle<T>) -> Result<T, RuwiError> {
    handle.join().or_else(|_| {
        Err(rerr!(
            RuwiErrorKind::FailedToSpawnThread,
            "Failed to spawn thread."
        ))
    })
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_run_ruwi() -> Result<(), ErrBox> {
    //     run_ruwi()
    // }
}
