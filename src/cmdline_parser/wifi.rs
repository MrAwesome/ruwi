use super::utils::*;
use super::{WIFI_CONNECT_TOKEN, WIFI_SELECT_TOKEN};

use crate::interface_management::ip_interfaces::WifiIPInterface;

use crate::enums::*;
use crate::rerr;
use crate::options::wifi::connect::WifiConnectOptions;
use crate::options::command::*;
use crate::options::wifi::select::WifiSelectOptions;
use crate::options::wifi::*;
use crate::options::interfaces::*;
use crate::errors::*;
use crate::options::*;
use crate::strum_utils::*;

use clap::ArgMatches;
use std::fmt::Debug;

pub(super) fn get_wifi_cmd(
    globals: GlobalOptions,
    maybe_wifi_matcher: Option<&ArgMatches>,
) -> Result<RuwiWifiCommand, RuwiError> {
    if let Some(wifi_matcher) = maybe_wifi_matcher {
        let wifi_opts = get_wifi_opts_impl(globals, wifi_matcher)?;
        let (subcommand_name, subcommand_matcher) = wifi_matcher.subcommand();

        let cmd = if subcommand_name == "" || subcommand_name == WIFI_CONNECT_TOKEN {
            RuwiWifiCommand::Connect(get_wifi_connect_opts(wifi_opts, subcommand_matcher)?)

        } else if subcommand_name == WIFI_SELECT_TOKEN {
            RuwiWifiCommand::Select(get_wifi_select_opts(wifi_opts, subcommand_matcher)?)
        } else {
            handle_cmdline_parsing_error(subcommand_name, subcommand_matcher)?
        };

        Ok(cmd)
    } else {
        get_default_wifi_command(globals)
    }
}

fn get_default_wifi_command(globals: GlobalOptions) -> Result<RuwiWifiCommand, RuwiError> {
    let interface = WifiIPInterface::find_first(&globals)?;
    Ok(RuwiWifiCommand::Connect(
        WifiConnectOptions::builder()
            .wifi(
                WifiOptions::builder()
                    .globals(globals)
                    .interface(interface)
                    .build(),
            )
            .build(),
    ))
}

fn get_wifi_connect_opts(
    wifi_opts: WifiOptions,
    maybe_connect_matcher: Option<&ArgMatches>,
) -> Result<WifiConnectOptions, RuwiError> {
    let connect_builder = WifiConnectOptions::builder().wifi(wifi_opts);
    let connect_opts = if let Some(connect_matcher) = maybe_connect_matcher {
        let force_ask_password = connect_matcher.is_present("force_ask_password");
        let given_essid = connect_matcher.value_of("essid").map(String::from);
        let given_encryption_key = connect_matcher.value_of("password").map(String::from);

        let auto_mode = if connect_matcher.is_present("auto") {
            AutoMode::KnownOrAsk
        } else {
            get_val_as_enum::<AutoMode>(&connect_matcher, "auto_mode")
        };

        let connect_via = get_val_as_enum::<WifiConnectionType>(&connect_matcher, "connect_via");

        connect_builder
            .connect_via(connect_via)
            .given_essid(given_essid)
            .given_encryption_key(given_encryption_key)
            .auto_mode(auto_mode)
            .force_ask_password(force_ask_password)
            .build()
    } else {
        connect_builder.build()
    };
    Ok(connect_opts)
}

fn get_wifi_select_opts(
    wifi_opts: WifiOptions,
    maybe_select_matcher: Option<&ArgMatches>,
) -> Result<WifiSelectOptions, RuwiError> {
    let select_builder = WifiSelectOptions::builder().wifi(wifi_opts);
    let select_opts = if let Some(select_matcher) = maybe_select_matcher {
        let auto_mode = if select_matcher.is_present("auto") {
            AutoMode::KnownOrAsk
        } else {
            get_val_as_enum::<AutoMode>(&select_matcher, "auto_mode")
        };

        select_builder.auto_mode(auto_mode).build()
    } else {
        select_builder.build()
    };
    Ok(select_opts)
}

fn get_wifi_opts_impl(
    globals: GlobalOptions,
    sub_m: &ArgMatches,
) -> Result<WifiOptions, RuwiError> {
    let scan_method = get_scan_method(sub_m);
    let force_synchronous_scan = sub_m.is_present("force_synchronous_scan");
    let ignore_known = sub_m.is_present("ignore_known");
    let interface = get_wifi_interface(sub_m, &globals)?;
    let scan_type = ScanType::Wifi(get_val_as_enum::<WifiScanType>(&sub_m, "scan_type"));
    validate_scan_method_and_type(&scan_method, &scan_type)?;

    let wifi_opts = WifiOptions::builder()
        .globals(globals)
        .scan_type(scan_type)
        .scan_method(scan_method)
        .interface(interface)
        .ignore_known(ignore_known)
        .force_synchronous_scan(force_synchronous_scan)
        .build();

    Ok(wifi_opts)
}

fn get_scan_method(m: &ArgMatches) -> ScanMethod {
    if let Some(filename) = m.value_of("input_file").map(String::from) {
        ScanMethod::FromFile(filename)
    } else if m.is_present("input_stdin") {
        ScanMethod::FromStdin
    } else {
        ScanMethod::ByRunning
    }
}

fn get_wifi_interface<O>(m: &ArgMatches, opts: &O) -> Result<WifiIPInterface, RuwiError>
where
    O: Global + Debug,
{
    Ok(match m.value_of("interface") {
        Some(given_ifname) => WifiIPInterface::new(given_ifname),
        None => WifiIPInterface::find_first(opts)?,
    })
}

// TODO: can this be expressed in the type system somehow?
fn validate_scan_method_and_type(
    scan_method: &ScanMethod,
    scan_type: &ScanType,
) -> Result<(), RuwiError> {
    match (scan_method, scan_type) {
        (ScanMethod::ByRunning, ScanType::Wifi(WifiScanType::RuwiJSON)) => Err(rerr!(
                RuwiErrorKind::InvalidScanTypeAndMethod,
                "There is currently no binary for providing JSON results, you must format them yourself and pass in via stdin or from a file.",
        )),
        (_, _) => Ok(()),
    }
}

