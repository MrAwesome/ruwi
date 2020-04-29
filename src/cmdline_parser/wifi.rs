use super::utils::*;
use super::{WIFI_CONNECT_TOKEN, WIFI_SELECT_TOKEN};

use crate::prelude::*;
use crate::options::command::*;
use crate::options::wifi::connect::WifiConnectOptions;
use crate::options::wifi::select::WifiSelectOptions;
use crate::options::wifi::*;
use crate::options::*;
use crate::service_detection::*;
use crate::strum_utils::*;

use clap::ArgMatches;

const CONNECT_VIA_TOKEN: &str = "connect_via";
const SCAN_TYPE_TOKEN: &str = "scan_type";

pub(super) fn get_wifi_cmd(
    globals: GlobalOptions,
    maybe_wifi_matcher: Option<&ArgMatches>,
) -> Result<RuwiWifiCommand, RuwiError> {
    let cmd = if let Some(wifi_matcher) = maybe_wifi_matcher {
        let wifi_opts = get_wifi_options(globals, wifi_matcher)?;

        let (subcommand_name, subcommand_matcher) = wifi_matcher.subcommand();
        if subcommand_name == "" || subcommand_name == WIFI_CONNECT_TOKEN {
            RuwiWifiCommand::Connect(get_wifi_connect_opts(wifi_opts, subcommand_matcher)?)
        } else if subcommand_name == WIFI_SELECT_TOKEN {
            RuwiWifiCommand::Select(get_wifi_select_opts(wifi_opts, subcommand_matcher)?)
        } else {
            handle_cmdline_parsing_error(subcommand_name, subcommand_matcher)?
        }
    } else {
        get_default_wifi_command(globals)?
    };
    Ok(cmd)
}

fn get_wifi_options(
    globals: GlobalOptions,
    wifi_matcher: &ArgMatches,
) -> Result<WifiOptions, RuwiError> {
    let scan_method = get_scan_method(wifi_matcher);
    let force_synchronous_scan = wifi_matcher.is_present("force_synchronous_scan");
    let ignore_known = wifi_matcher.is_present("ignore_known");
    let given_interface_name = wifi_matcher.value_of("interface").map(String::from);
    let scan_type = if wifi_matcher.is_present(SCAN_TYPE_TOKEN) {
        get_val_as_enum::<WifiScanType>(&wifi_matcher, SCAN_TYPE_TOKEN)
    } else {
        let checker = SystemCheckerReal::new(&globals);
        WifiScanType::choose_best_from_system(&checker, SCAN_TYPE_TOKEN)
    };

    let wifi_opts = WifiOptions::builder()
        .globals(globals)
        .scan_type(scan_type)
        .scan_method(scan_method)
        .given_interface_name(given_interface_name)
        .ignore_known(ignore_known)
        .force_synchronous_scan(force_synchronous_scan)
        .build();
    validate_wifi_options(wifi_opts)
}

fn get_default_wifi_command(globals: GlobalOptions) -> Result<RuwiWifiCommand, RuwiError> {
    let wifi_opts = WifiOptions::builder().globals(globals).build();
    let connect_opts = WifiConnectOptions::builder().wifi(wifi_opts).build();
    Ok(RuwiWifiCommand::Connect(connect_opts))
}

fn get_wifi_connect_opts(
    wifi_opts: WifiOptions,
    maybe_connect_matcher: Option<&ArgMatches>,
) -> Result<WifiConnectOptions, RuwiError> {
    let connect_builder = WifiConnectOptions::builder();
    let connect_opts = if let Some(connect_matcher) = maybe_connect_matcher {
        let force_ask_password = connect_matcher.is_present("force_ask_password");
        let given_essid = connect_matcher.value_of("essid").map(String::from);
        let given_encryption_key = connect_matcher.value_of("password").map(String::from);

        let auto_mode = if connect_matcher.is_present("auto") {
            AutoMode::KnownOrAsk
        } else {
            get_val_as_enum::<AutoMode>(&connect_matcher, "auto_mode")
        };

        let connect_via = if connect_matcher.is_present(CONNECT_VIA_TOKEN) {
            get_val_as_enum::<WifiConnectionType>(&connect_matcher, CONNECT_VIA_TOKEN)
        } else {
            let checker = SystemCheckerReal::new(&wifi_opts);
            WifiConnectionType::choose_best_from_system(&checker, CONNECT_VIA_TOKEN)
        };

        connect_builder
            .wifi(wifi_opts)
            .connect_via(connect_via)
            .given_essid(given_essid)
            .given_encryption_key(given_encryption_key)
            .auto_mode(auto_mode)
            .force_ask_password(force_ask_password)
            .build()
    } else {
        connect_builder.wifi(wifi_opts).build()
    };

    validate_wifi_connect_options(connect_opts)
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
    validate_wifi_select_options(select_opts)
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

// TODO: unit test these validation functions in option creation
fn validate_wifi_options(options: WifiOptions) -> Result<WifiOptions, RuwiError> {
    let scan_method = options.get_scan_method();
    let scan_type = options.get_scan_type();
    match (scan_method, scan_type) {
        (ScanMethod::ByRunning, WifiScanType::RuwiJSON) => Err(rerr!(
            RuwiErrorKind::InvalidScanTypeAndMethod,
            "There is currently no binary for providing JSON results, you must format them yourself and pass in via stdin or from a file.",
        )),
        _ => Ok(options),
    }
}

fn validate_wifi_connect_options(
    options: WifiConnectOptions,
) -> Result<WifiConnectOptions, RuwiError> {
    let scan_method = options.get_scan_method();
    let scan_type = options.get_scan_type();
    let connect_via = options.get_connect_via();
    match (scan_method, connect_via) {
        (ScanMethod::ByRunning, WifiConnectionType::Nmcli) => {
            if let WifiScanType::Nmcli = scan_type {
                Ok(options)
            } else {
                Err(rerr!(
                    RuwiErrorKind::InvalidScanTypeAndConnectType,
                    "Non-nmcli scan types do not work when connect_via is set to nmcli, as nmcli needs the NetworkManager service enabled while it looks for known networks. You can pass in results from another scanning program with -I or -F, but most likely you just want to add \"-s nmcli\" to wifi."

                ))
            }
        }
        _ => Ok(options),
    }
}

fn validate_wifi_select_options(
    options: WifiSelectOptions,
) -> Result<WifiSelectOptions, RuwiError> {
    Ok(options)
}
