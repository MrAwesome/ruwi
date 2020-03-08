// Ensure serious cleanliness:
#![warn(clippy::pedantic)]
// But these are a bit too pedantic:
#![allow(clippy::similar_names)]
#![allow(clippy::missing_errors_doc)]

extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate strum;
extern crate serde_derive;
extern crate strum_macros;
extern crate typed_builder;

#[macro_use]
pub(crate) mod macros;

// TODO: collapse these into subdirs
pub(crate) mod annotate_networks;
pub(crate) mod bluetooth;
pub(crate) mod check_known_identifiers;
pub(crate) mod cmdline_parser;
pub(crate) mod configure_network;
pub(crate) mod connect;
pub(crate) mod encryption_key;
pub(crate) mod enums;
pub(crate) mod errors;
pub(crate) mod find_known_network_names;
pub mod interface_management;
pub(crate) mod netctl_config_writer;
pub(crate) mod options;
pub(crate) mod parse;
pub(crate) mod run_commands;
pub(crate) mod runner;
pub(crate) mod select;
pub(crate) mod service_management;
pub(crate) mod sort_networks;
pub(crate) mod structs;
pub(crate) mod strum_utils;
pub(crate) mod synchronous_retry_logic;
pub(crate) mod utils;
pub(crate) mod wifi_scan;
pub(crate) mod wpa_cli_initialize;

use cmdline_parser::get_command_from_command_line;
use errors::RuwiError;

// Arch dependencies: wireless_tools, netctl, iw, bluetooth things?, fzf
// Arch optional dependencies: dmenu, iwconfig, NetworkManager,


// TODO(high): include netctl profile name with annotated wired/wireless networks, for connecting to known networks with non-ruwified names - have known netctl networks return essid + config name, for matching/annotation with config name
// TODO(high): `clear` should be `wifi clear`? or at least call into it? i guess wired and wireless
// may use the same services. bluetooth also will have services i suppose. should ip/bt be
// different service types?
// TODO(high): implement speed/connection/dns test - see `nmcli networking connectivity` for networkmanager mode
// TODO(high): implement wired
// TODO(high): implement bluetooth
// TODO(high): write benchmark tests: ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/liballoc/benches/slice.rs
// TODO(mid): add colors to output / use a real logging library
// TODO(mid): ability to do -o "wired.connect_via=netctl", overriding config file entries
// TODO(mid): add a "list seen networks" mode?
// TODO(mid): kill, or suggest killing, wpa_supplicant if netctl fails to connect (clear does this, can you just suggest clear in error messages?)
// TODO(mid): have `ruwi -a` detect wired (can you detect a plugged-in ethernet?), try to connect to it, then try wifi -a if not
// TODO(low): kill wpa_supplicant if trying to use raw iw or networkmanager
// TODO(low): flag to disable looking for known networks
// TODO(wishlist): `ruwi wifi get_default_interface` and/or `ruwi wifi select_interface`?
// TODO(wishlist): JSON output for `select`
// TODO(wishlist): implement json scan output/input mode
// TODO(wishlist): find a generalized way to do x notifications, for dmenu mode, use to surface failures
// TODO(wishlist): connection/scan type: wicd-cli
// TODO(wishlist): fzf keyboard shortcuts for getting more info about a network?
// TODO(later): make sure fzf and dmenu are listed as dependencies

pub fn run_ruwi() -> Result<(), RuwiError> {
    let command = get_command_from_command_line()?;
    command.run()
}
