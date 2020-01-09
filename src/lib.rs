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
pub(crate) mod wifi_scan;
pub(crate) mod select_network;
pub(crate) mod select_utils;
pub(crate) mod sort_networks;
pub(crate) mod structs;
pub(crate) mod synchronous_retry_logic;
pub(crate) mod service_management;
#[macro_use]
pub(crate) mod macros;
pub(crate) mod strum_utils;
pub(crate) mod wpa_cli_initialize;

use cmdline_parser::*;
use runner::run_ruwi_using_state_machine;
use structs::*;

// TODO(high): implement speed/connection/dns test
// TODO(high): stop/start the relevant services (in particular, stop networkmanager if it's running before trying to do netctl things) - - pkill wpa_supplicant, systemctl stop NetworkManager, etc etc etc
// TODO(high): if networkmanager is used, start it up before going - same with netctl. possibly also stop
// TODO(high): fix error messages. -F kfdjsalkf will give "ERR: entity not found"
// TODO(high): write benchmark tests: https://doc.rust-lang.org/1.2.0/book/benchmark-tests.html
// TODO(mid): add colors to output / use a real logging library
// TODO(mid): add a "list seen networks" mode?
// TODO(mid): have known netctl networks return essid, for matching/annotation with config name
// TODO(low): kill wpa_supplicant if trying to use raw iw or networkmanager
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
    run_ruwi_using_state_machine(&command, options)
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
