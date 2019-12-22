// Prerequisite packages:
// iw
// wpa_supplicant?

// #![deny(warnings)]
// #![deny(clippy::all)]
#![warn(clippy::all)]

extern crate clap;
extern crate regex;
extern crate strum;
extern crate strum_macros;

pub(crate) mod annotate_networks;
pub(crate) mod cmdline_parser;
pub(crate) mod configure_network;
pub(crate) mod connect;
pub(crate) mod display_network_for_selection;
pub(crate) mod encryption_key;
pub(crate) mod find_known_network_names;
pub(crate) mod get_default_interface;
pub(crate) mod interface_management;
pub(crate) mod netctl_config_writer;
pub(crate) mod parse;
pub(crate) mod run_commands;
pub(crate) mod scan;
pub(crate) mod select_network;
pub(crate) mod select_utils;
pub(crate) mod sort_networks;
pub(crate) mod structs;
pub(crate) mod synchronous_retry_logic;
#[macro_use]
pub(crate) mod macros;
pub(crate) mod strum_utils;
pub(crate) mod wpa_cli_initialize;

use annotate_networks::*;
use cmdline_parser::*;
use configure_network::*;
use connect::*;
use encryption_key::*;
use find_known_network_names::*;
use parse::*;
use scan::*;
use select_network::*;
use sort_networks::*;
use std::thread;
use structs::*;
use synchronous_retry_logic::*;

// TODO(high): mock out known-network finding in integration tests, ensure it isn't happening in unit tests. --dry-run flag? behave differently when not run as root? something.
// TODO(high): stop/start the relevant services (in particular, stop networkmanager if it's running before trying to do netctl things) - - pkill wpa_supplicant, systemctl stop NetworkManager, etc etc etc
// TODO(high): figure out how to unit test / mock command calls
// TODO(high): if networkmanager is used, start it up before going - same with netctl. possibly also stop
// TODO(mid): add colors to output / use a real logging library
// TODO(mid): figure out what to do about existing netctl configs
// TODO(mid): add a "list seen networks" mode?
// TODO(mid): list all inputs, and create appropriate command line flags - Options should not be created with filenames, but with the appropriate structs already populated during the cmdline parsing stage (knownnetwork names, password, etc read from a file)? or are filenames fine? i don't think it matters, just affects how you test
// TODO(low): kill wpa_supplicant if trying to use raw iw or networkmanager
// TODO(low): integration test with -e and -p
// TODO(low): move majority of code from here into another file
// TODO(low): flag to disable looking for known networks
// TODO(wishlist): if there are multiple interfaces seen by 'iw dev', bring up selection, otherwise pick the default
// TODO(wishlist): implement json scan output mode
// TODO(wishlist): find a generalized way to do x notifications, for dmenu mode, use to surface failures
// TODO(wishlist): determine whether to use dmenu/fzf/etc based on terminal/X
// TODO(wishlist): connection/scan type: wicd-cli
// TODO(wishlist): fzf keyboard shortcuts for getting more info about a network?
// TODO(wishlist): containers which emulate systems on which ruwi should act a particular way (interface name, etc)
// TODO(later): make sure fzf and dmenu are listed as dependencies
// TODO(think): instead of functions which take options, make a big struct/impl? maybe more than one?
// TODO(think): add a -w/--wait or --verify or something to attempt to connect to google/etc?
// TODO(think): make -a the default?

pub fn run_ruwi() -> Result<(), RuwiError> {
    let options = &get_options()?;
    // let command = options.command;
    // match command {
    //      RuwiCommand::Connect => {}
    //      RuwiCommand::Select => {}
    //      RuwiCommand::List => {}
    //      RuwiCommand::DumpJSON => {}
    //      RuwiCommand::Disconnect => {}
    // }
    // This is the primary run type / command. What are others?
    {
        let selected_network = use_given_or_scan_and_select_network(options)?;
        let encryption_key = possibly_get_encryption_key(options, &selected_network)?;
        let _output_result =
            possibly_configure_network(options, &selected_network, &encryption_key)?;
        let _connection_result = connect_to_network(options, &selected_network, &encryption_key)?;
    }
    Ok(())
}

fn use_given_or_scan_and_select_network(
    options: &Options,
) -> Result<AnnotatedWirelessNetwork, RuwiError> {
    match &options.given_essid {
        Some(essid) => get_network_from_given_essid(options, essid),
        None => scan_and_select_network_with_retry(options),
    }
}

fn get_network_from_given_essid(
    options: &Options,
    essid: &str,
) -> Result<AnnotatedWirelessNetwork, RuwiError> {
    let is_known = find_known_network_names(options.clone())?.contains(essid);
    let is_encrypted = options.given_encryption_key.is_some();
    Ok(AnnotatedWirelessNetwork::from_essid(
        essid.into(),
        is_known,
        is_encrypted,
    ))
}

fn scan_and_select_network_with_retry(
    options: &Options,
) -> Result<AnnotatedWirelessNetwork, RuwiError> {
    match scan_and_select_network(options) {
        Err(err) => match err.kind {
            RuwiErrorKind::RefreshRequested => scan_and_select_network_with_retry(
                &options.with_synchronous_retry(SynchronousRetryType::ManuallyRequested),
            ),
            RuwiErrorKind::RetryWithSynchronousScan => scan_and_select_network(
                &options.with_synchronous_retry(SynchronousRetryType::Automatic),
            ),
            _ => Err(err),
        },
        x => x,
    }
}

fn scan_and_select_network(options: &Options) -> Result<AnnotatedWirelessNetwork, RuwiError> {
    let annotated_networks = scan_parse_and_annotate_networks_with_retry(options)?;
    let sorted_networks = sort_and_filter_networks(options, annotated_networks);
    select_network(options, &sorted_networks)
}

// In automatic mode, we want to make a strong attempt to prioritize known networks.
// Cached results are often out of date, so if no known networks are seen, trust our
// user that they should be, and re-run the scan in synchronous mode.
// This logic lives so high in the stack because known status isn't available
// until we've found the known network names and used them to annotate the seen networks.
fn scan_parse_and_annotate_networks_with_retry(
    options: &Options,
) -> Result<AnnotatedNetworks, RuwiError> {
    let annotated_networks_res = scan_parse_and_annotate_networks(options);

    if let Ok(annotated_networks) = &annotated_networks_res {
        // TODO: possibly just rerun everything in synchronous mode if any problems are encountered?
        if should_retry_with_synchronous_scan(options, &annotated_networks) {
            eprintln!("[NOTE]: No known networks seen in auto mode using cached scan results. Manually scanning now. ");
            return Err(rerr!(
                RuwiErrorKind::RetryWithSynchronousScan,
                "Attempting synchonous retry."
            ));
        }
    }
    Ok(annotated_networks_res?)
}

fn scan_parse_and_annotate_networks(options: &Options) -> Result<AnnotatedNetworks, RuwiError> {
    let (known_network_names, scan_result) = gather_data(options)?;
    let parse_results = parse_result(options, &scan_result)?;
    let annotated_networks =
        annotate_networks(options, &parse_results.seen_networks, &known_network_names);

    Ok(annotated_networks)
}

fn gather_data(options: &Options) -> Result<(KnownNetworkNames, ScanResult), RuwiError> {
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
    //
    // #[test]
    // fn test_run_ruwi() -> Result<(), RuwiError> {
    //     run_ruwi()
    // }
}
