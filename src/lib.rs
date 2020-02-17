// Ensure serious cleanliness:
#![warn(clippy::pedantic)]
// But this one is a bit too pedantic:
#![allow(clippy::similar_names)]

extern crate clap;
extern crate strum;
extern crate strum_macros;
extern crate typed_builder;

pub(crate) mod annotate_networks;
pub(crate) mod check_known_identifiers;
pub(crate) mod cmdline_parser;
pub(crate) mod configure_network;
pub(crate) mod connect;
pub(crate) mod encryption_key;
pub(crate) mod enums;
pub(crate) mod errors;
pub(crate) mod find_known_network_names;
pub(crate) mod get_default_interface;
pub(crate) mod interface_management;
pub(crate) mod netctl_config_writer;
pub(crate) mod options;
pub(crate) mod parse;
pub(crate) mod run_commands;
pub(crate) mod runner;
pub(crate) mod select;
pub(crate) mod service_management;
pub(crate) mod sort_networks;
pub(crate) mod structs;
pub(crate) mod synchronous_retry_logic;
pub(crate) mod wifi_scan;
#[macro_use]
pub(crate) mod macros;
pub(crate) mod strum_utils;
pub(crate) mod wpa_cli_initialize;

use cmdline_parser::*;
use errors::*;
use options::*;

// Arch dependencies: netctl, iw, bluetooth things?, fzf
// Arch optional dependencies: dmenu, iwconfig, NetworkManager,

// TODO(high): implement speed/connection/dns test - `nmcli networking connectivity` for networkmanager mode
// TODO(high): stop/start the relevant services (in particular, stop networkmanager if it's running before trying to do netctl things) - - pkill wpa_supplicant, systemctl stop NetworkManager, etc etc etc (add "conflicting services" to src/service_management.rs )
// TODO(high): fix error messages. -F kfdjsalkf will give "ERR: entity not found"
// TODO(high): write benchmark tests: ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/liballoc/benches/slice.rs
// TODO(mid): add colors to output / use a real logging library
// TODO(mid): add a "list seen networks" mode?
// TODO(mid): have known netctl networks return essid, for matching/annotation with config name
// TODO(mid): kill, or suggest killing, wpa_supplicant if netctl fails to connect
// TODO(low): kill wpa_supplicant if trying to use raw iw or networkmanager
// TODO(low): flag to disable looking for known networks
// TODO(low): mark as linux-only
// TODO(wishlist): if there are multiple interfaces seen by 'iw dev', bring up selection, otherwise pick the default
// TODO(wishlist): implement json scan output mode
// TODO(wishlist): find a generalized way to do x notifications, for dmenu mode, use to surface failures
// TODO(wishlist): connection/scan type: wicd-cli
// TODO(wishlist): fzf keyboard shortcuts for getting more info about a network?
// TODO(later): make sure fzf and dmenu are listed as dependencies
// TODO(think): instead of functions which take options, make a big struct/impl? maybe more than one?
// TODO(think): add a -w/--wait or --verify or something to attempt to connect to google/etc?

pub fn run_ruwi() -> Result<(), RuwiError> {
    let command = get_command()?;
    command.run()
}
