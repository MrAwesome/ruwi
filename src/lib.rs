// Ensure serious cleanliness:
#![warn(clippy::pedantic)]
// But these are a bit too pedantic:
#![allow(clippy::similar_names)]
#![allow(clippy::missing_errors_doc)]

#![forbid(unsafe_code)]

extern crate clap;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate strum;
extern crate nix;
#[cfg(test)]
extern crate mockall;
#[cfg(test)]
extern crate paste;
extern crate strum_macros;
extern crate typed_builder;

#[macro_use]
pub(crate) mod macros;

// TODO: collapse these into subdirs
pub(crate) mod annotate_networks;
pub(crate) mod bluetooth;
pub(crate) mod common;
pub(crate) mod cmdline_parser;
pub(crate) mod configure_network;
pub(crate) mod connect;
pub(crate) mod encryption_key;
pub(crate) mod enums;
pub mod errors;
// TODO: instead of making this public, have a "public constants" module
pub(crate) mod interface_management;
pub(crate) mod known_networks;
pub(crate) mod netctl;
pub(crate) mod options;
pub(crate) mod parse;
pub(crate) mod run_commands;
pub(crate) mod runner;
pub(crate) mod select;
pub(crate) mod service_detection;
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

// Arch dependencies: wireless_tools, netctl, iw, bluetooth things?, iptools
// Arch optional dependencies: dmenu, NetworkManager, fzf

// TODO(urgent): fix netctl wired connections, create selector/reader/writer
// TODO(urgent): fix wpa_cli service start interface (associate interface with service enum?)
// TODO(high): use crate::common::*; instead of misc imports
// TODO(high): add `clear` success messages
// TODO(high): make `ruwi` with no arguments still go through system checks
// TODO(high): include netctl profile name with annotated wired/wireless networks, for connecting to known networks with non-ruwified names - have known netctl networks return essid + config name, for matching/annotation with config name
// TODO(high): implement Selectable for netctl profiles, for wired connections (and wifi as well, since that seems like a reasonable use case)
// TODO(high): `clear` should be `wifi clear`? or at least call into it? i guess wired and wireless may use the same services. bluetooth also will have services i suppose. should ip/bt be different service types?
// TODO(high): implement speed/connection/dns test - see `nmcli networking connectivity` for networkmanager mode
// TODO(high): implement bluetooth
// TODO(high): write benchmark tests: ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/liballoc/benches/slice.rs
// TODO(mid): add colors to output / use a real logging library / set debugging levels
// TODO(mid): ability to do -o "wired.connect_via=netctl", overriding config file entries
// TODO(mid): add a "list seen networks" mode?
// TODO(mid): use string_container where you would normally pass around String or an existing less-nice string container
// TODO(mid): kill, or suggest killing, wpa_supplicant if netctl fails to connect (clear does this, can you just suggest clear in error messages?)
// TODO(mid): have `ruwi -a` detect wired, try to connect to it, then try wifi -a if not. check "/sys/class/net/{IFNAME}/operstate" after bringing up the interface
// TODO(low): kill wpa_supplicant if trying to use raw iw or networkmanager
// TODO(low): flag to disable looking for known networks
// TODO(low): standardize quotes in help text (search codebase for "manually")
// TODO(low): remove Default trait for structs which use TypedBuilder
// TODO(low): use TryFrom instead of custom conversion methods
// TODO(low): use a custom Result type to reduce Result<_, RuwiError> boilerplate
// TODO(wishlist): `ruwi wifi get_default_interface` and/or `ruwi wifi select_interface`?
// TODO(wishlist): JSON output for `select`
// TODO(wishlist): implement json scan output/input mode
// TODO(wishlist): find a generalized way to do x notifications, for dmenu mode, use to surface failures
// TODO(wishlist): connection/scan type: wicd-cli
// TODO(wishlist): fzf keyboard shortcuts for getting more info about a network?
// TODO(later): make sure fzf and dmenu are listed as dependencies

pub fn run_ruwi_cli() -> Result<(), RuwiError> {
    let command = get_command_from_command_line()?;
    command.run()
}
