// Ensure serious cleanliness:
#![warn(clippy::pedantic)]
// But this one is a bit too pedantic:
#![allow(clippy::similar_names)]
// And this catches some long test functions which are fine.
#![allow(clippy::too_many_lines)]

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
pub(crate) mod runner;
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
use runner::run_ruwi_using_state_machine;
use scan::*;
use select_network::*;
use sort_networks::*;
use std::thread;
use structs::*;
use synchronous_retry_logic::*;

// TODO(high): implement speed/connection/dns test
// TODO(high): stop/start the relevant services (in particular, stop networkmanager if it's running before trying to do netctl things) - - pkill wpa_supplicant, systemctl stop NetworkManager, etc etc etc
// TODO(high): if networkmanager is used, start it up before going - same with netctl. possibly also stop
// TODO(high): fix error messages. -F kfdjsalkf will give "ERR: entity not found"
// TODO(high): write benchmark tests: https://doc.rust-lang.org/1.2.0/book/benchmark-tests.html
// TODO(mid): add colors to output / use a real logging library
// TODO(mid): add a "list seen networks" mode?
// TODO(mid): have known netctl networks return essid, for matching/annotation with config name
// TODO(low): kill wpa_supplicant if trying to use raw iw or networkmanager
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
    let command = RuwiCommand::default();
    let options = &get_options()?;

    //eprintln!("[FIXME] Attempting state machine run first!");
    run_ruwi_using_state_machine(&command, options)
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
    let is_known = find_known_network_names(options)?.contains(essid);
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
                &options.with_synchronous_retry(SynchronousRescanType::ManuallyRequested),
            ),
            RuwiErrorKind::RetryWithSynchronousScan => scan_and_select_network(
                &options.with_synchronous_retry(SynchronousRescanType::Automatic),
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
    let annotated_networks = scan_parse_and_annotate_networks(options)?;

    if should_retry_with_synchronous_scan(options, &annotated_networks) {
        eprintln!("[NOTE]: No known networks seen in auto mode using cached scan results. Manually scanning now. ");
        return Err(rerr!(
            RuwiErrorKind::RetryWithSynchronousScan,
            "Attempting synchonous retry."
        ));
    }

    Ok(annotated_networks)
}

fn scan_parse_and_annotate_networks(options: &Options) -> Result<AnnotatedNetworks, RuwiError> {
    let (known_network_names, scan_result) = gather_wifi_network_data(options)?;
    let parse_results = parse_result(options, &scan_result)?;
    let annotated_networks =
        annotate_networks(options, &parse_results.seen_networks, &known_network_names);

    Ok(annotated_networks)
}

fn gather_wifi_network_data(
    options: &Options,
) -> Result<(KnownNetworkNames, ScanResult), RuwiError> {
    let (opt1, opt2) = (options.clone(), options.clone());
    let get_nw_names = thread::spawn(move || find_known_network_names(&opt1));
    let get_scan_results = thread::spawn(move || wifi_scan(&opt2));

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
