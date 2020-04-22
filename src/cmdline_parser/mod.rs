mod utils;
mod wifi;
mod wired;

use utils::*;
use wifi::get_wifi_cmd;
use wired::get_wired_cmd;

use crate::enums::*;
use crate::errors::*;
use crate::options::command::*;
use crate::options::*;
use crate::strum_utils::*;

use std::env;

use clap::{App, Arg, ArgMatches, SubCommand};
use strum::AsStaticRef;

const CLEAR_TOKEN: &str = "clear";
const WIFI_TOKEN: &str = "wifi";
const WIRED_TOKEN: &str = "wired";

const WIRED_CONNECT_TOKEN: &str = "connect";

const WIFI_SELECT_TOKEN: &str = "select";
const WIFI_CONNECT_TOKEN: &str = "connect";

const PRETEND_TO_BE_ROOT_TOKEN: &str = "PRETEND_TO_BE_ROOT";

const CMDLINE_BAILOUT_TOKEN: &str = "ONLY_PARSE_CMDLINE";
const CMDLINE_BAILOUT_MSG: &str = "Command line parsing was successful, but cmdline bailout was requested!";

// TODO: find a better place for interface to live (perhaps not on options at all?)
// TODO: add help for connect and clear and select!
// TODO: respect force_ask_password
// TODO: fail if not run as root
// TODO: use subcommands for conuigurations of options, but still go through all functions always?
//       or just run certain functions for certain subcommands?
// TODO: for netctl selection options, alias --identifier and --filename
#[allow(clippy::too_many_lines)]
fn get_arg_app<'a, 'b>() -> App<'a, 'b> {
    // TODO: use .aliases for commands
    // TODO: use .global(true) once there's a supported function in clap for recursively checking subcommands
    let debug = Arg::with_name("debug")
        .short("d")
        .long("debug")
        .help("Print out debug information on what ruwi sees.");

    let dry_run = Arg::with_name("dry_run")
        .short("D")
        .long("dry-run")
        .help("Don't write to or read from any files, interfaces, or networks. Mostly just useful for integration tests.");

    let input_file = Arg::with_name("input_file")
        .short("F")
        .long("input-file")
        .takes_value(true)
        .help("Instead of running a scan, use scan results from specified file.");

    let input_stdin = Arg::with_name("input_stdin")
        .short("I")
        .long("input-stdin")
        .help("Instead of running a scan, use scan results from stdin.");

    let auto = Arg::with_name("auto").short("a").long("auto").help(
        "Connect to the strongest known network seen. Will prompt for selection if no known networks are seen. Shorthand for `-A known_or_ask`. Takes precedence over `-A`.",
    );

    let auto_mode = Arg::with_name("auto_mode")
        .short("A")
        .long("auto-mode")
        .takes_value(true)
        .default_value(&AutoMode::default().as_static())
        .possible_values(&possible_string_vals::<AutoMode, _>())
        .help("The auto mode to use.");

    let force_synchronous = Arg::with_name("force_synchronous_scan")
        .short("f")
        .long("force-sync")
        .help("Do not look at cached results, always scan for networks when run.");

    let ignore_known = Arg::with_name("ignore_known")
        .long("ignore-known")
        .help("Do not try to determine if networks are already known. Passwords will be requested always in this mode.");

    let networking_interface = Arg::with_name("interface")
        .short("i")
        .long("interface")
        .takes_value(true)
        .help("The Linux networking interface (e.g. wlp3s0, eth0) to use. Will attempt to use `ip link show` to infer it, if none given.");

    let essid = Arg::with_name("essid")
        .short("e")
        .long("essid")
        .takes_value(true)
        .help("Manually specify wireless network name (aka SSID or ESSID). Will be asked for if not given. Assumes the network is open, use `-P` to prompt for password or `-p`  to pass one in directly.");

    let password = Arg::with_name("password")
        .short("p")
        .long("password")
        .takes_value(true)
        .help("Manually specify encryption key (aka password). To read from a file, try \"$(cat your/file/name)\". Will replace any existing password for the selected or given network.");

    let force_ask_password = Arg::with_name("force_ask_password")
        .short("P")
        .long("force-ask-password")
        .help("Will always prompt for a password when selecting a network, or passing an SSID with `-e`. Ignored with `-p`, or on connection/output types where a password wouldn't be used anyway.");

    let wifi_scan_type = Arg::with_name("scan_type")
        .short("s")
        .long("scan-type")
        .takes_value(true)
        .possible_values(&possible_string_vals::<WifiScanType, _>())
        .help("The wifi scanning program to use to get results.");

    let selection_method = Arg::with_name("selection_method")
        .short("m")
        .long("selection-method")
        .takes_value(true)
        .default_value(&SelectionMethod::default().as_static())
        .possible_values(&possible_string_vals::<SelectionMethod, _>())
        .help("The program to use to prompt for input.");

    let wifi_connect_via = Arg::with_name("connect_via")
        .short("c")
        .long("connect-via")
        .takes_value(true)
        .possible_values(&possible_string_vals::<WifiConnectionType, _>())
        .help("Which network management suite to use to connect to the selected SSID on the given interface. Will attempt to automatically determine the best connector based on which programs are currently running/installed on the system.");

    let wired_connect_via = Arg::with_name("connect_via")
        .short("c")
        .long("connect-via")
        .takes_value(true)
        .default_value(&RawInterfaceConnectionType::default().as_static())
        .possible_values(&possible_string_vals::<RawInterfaceConnectionType, _>())
        .help("Which network management suite to use to connect on the given interface.");

    App::new("Ruwi")
        .version("0.2")
        .author("Glenn Hope <glenn.alexander.hope@gmail.com>")
        .about("Combines existing network management layers (netctl, NetworkManager, wicd) and selection utilities (fzf, dmenu) to find, select, configure, and connect to wireless networks.")
        .arg(debug)
        .arg(dry_run)
        .arg(selection_method)
        .subcommand(SubCommand::with_name(CLEAR_TOKEN).help("Stop all managed networking services (netctl, NetworkManager, wpa_supplicant, etc.)"))
        .subcommand(SubCommand::with_name(WIRED_TOKEN)
            .arg(networking_interface.clone())
            .subcommand(SubCommand::with_name(WIRED_CONNECT_TOKEN)
                .arg(wired_connect_via)
            )
        )
        .subcommand(SubCommand::with_name(WIFI_TOKEN)
            .arg(ignore_known)
            .arg(input_file)
            .arg(input_stdin)
            .arg(force_synchronous)
            .arg(networking_interface)
            .arg(wifi_scan_type)
            .subcommand(SubCommand::with_name(WIFI_CONNECT_TOKEN)
                .arg(auto.clone())
                .arg(auto_mode.clone())
                .arg(wifi_connect_via)
                .arg(essid)
                .arg(force_ask_password)
                .arg(password))
            .subcommand(SubCommand::with_name(WIFI_SELECT_TOKEN)
                .arg(auto)
                .arg(auto_mode)
            )
        )
}

pub(crate) fn get_command_from_command_line() -> Result<RuwiCommand, RuwiError> {
    let m = get_arg_app().get_matches();
    get_command_from_command_line_impl(&m)
}

// TODO: return an enum of options/commands types
fn get_command_from_command_line_impl(m: &ArgMatches) -> Result<RuwiCommand, RuwiError> {
    let pretend_to_be_root = is_env_var_set_to_1(PRETEND_TO_BE_ROOT_TOKEN);

    let debug = m.is_present("debug");
    let selection_method = get_val_as_enum::<SelectionMethod>(&m, "selection_method");

    let dry_run = m.is_present("dry_run");
    if dry_run {
        // TODO: actually use cached results, or remove that from the message here.
        eprintln!("[NOTE]: Running in dryrun mode! Will not run any external commands (besides the requested prompt command) or write/read configs on disk, and will only use cached scan results.");
    }

    let globals = GlobalOptions::builder()
        .debug(debug)
        .dry_run(dry_run)
        .selection_method(selection_method)
        .pretend_to_be_root(pretend_to_be_root)
        .build();

    let (command_name, maybe_cmd_matcher) = m.subcommand();
    let cmd = if command_name == WIFI_TOKEN || command_name == "" {
        RuwiCommand::Wifi(get_wifi_cmd(globals, maybe_cmd_matcher)?)
    } else if command_name == CLEAR_TOKEN {
        RuwiCommand::Clear(globals)
    } else if command_name == WIRED_TOKEN {
        RuwiCommand::Wired(get_wired_cmd(globals, maybe_cmd_matcher)?)
    } else {
        handle_cmdline_parsing_error(command_name, maybe_cmd_matcher)?
    };

    if debug {
        dbg![&cmd];
    }

    only_parse_cmdline_check()?;
    Ok(cmd)
}

fn is_env_var_set_to_1(name: &str) -> bool {
    let var_res = env::var(name);
    match var_res {
        Ok(var_val) => var_val == "1",
        Err(_) => false,
    }
}

fn only_parse_cmdline_check() -> Result<(), RuwiError> {
    if is_env_var_set_to_1(CMDLINE_BAILOUT_TOKEN) {
        Err(rerr!(
            RuwiErrorKind::OnlyParseCmdlineBailout,
            CMDLINE_BAILOUT_MSG,
        ))
    } else {
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::options::interfaces::*;
    use crate::options::wifi::connect::WifiConnectOptions;
    use crate::options::wired::connect::WiredConnectOptions;
    use crate::rerr;

    use clap::ArgMatches;
    use strum::IntoEnumIterator;

    static FAKE_BINARY_NAME: &str = "fake_binary_name";

    fn construct_args<'a>(args: &'a [&str]) -> Vec<&'a str> {
        let mut vec_args = vec![FAKE_BINARY_NAME];
        for x in args {
            vec_args.push(x);
        }
        vec_args
    }

    fn get_matches<'a>(args: &[&str]) -> ArgMatches<'a> {
        get_arg_app().get_matches_from(construct_args(args))
    }

    fn get_matches_safe<'a>(args: &[&str]) -> Result<ArgMatches<'a>, clap::Error> {
        get_arg_app().get_matches_from_safe(construct_args(args))
    }

    fn getopts(args: &[&str]) -> RuwiCommand {
        dbg!(args);
        get_command_from_command_line_impl(&get_matches(args)).unwrap()
    }

    fn expect_clear_opts(cmd: RuwiCommand) -> GlobalOptions {
        if let RuwiCommand::Clear(opts) = cmd {
            opts
        } else {
            panic!("Expected command to be 'clear', but got: {:?}", cmd);
        }
    }

    fn expect_wifi_connect_opts(cmd: RuwiCommand) -> WifiConnectOptions {
        if let RuwiCommand::Wifi(RuwiWifiCommand::Connect(opts)) = cmd {
            opts
        } else {
            panic!("Expected command to be 'wifi connect', but got: {:?}", cmd);
        }
    }

    fn expect_wired_connect_opts(cmd: RuwiCommand) -> WiredConnectOptions {
        if let RuwiCommand::Wired(RuwiWiredCommand::Connect(opts)) = cmd {
            opts
        } else {
            panic!("Expected command to be 'wired connect', but got: {:?}", cmd);
        }
    }

    fn getopts_safe(args: &[&str]) -> Result<RuwiCommand, RuwiError> {
        dbg!(&args);
        let matcher = get_matches_safe(args).map_err(|e| {
            rerr!(
                RuwiErrorKind::CmdLineOptParserSafeFailedInTest,
                e.to_string()
            )
        });
        dbg!(&matcher);
        let res = get_command_from_command_line_impl(&matcher?);
        dbg!(&res);
        res
    }

    fn test_fail_incorrect_option(args: &[&str]) {
        dbg!(&args);
        let cmd_res = getopts_safe(args);
        dbg!(&cmd_res);
        if let Err(err) = cmd_res {
            assert_eq![err.kind, RuwiErrorKind::CmdLineOptParserSafeFailedInTest];
        } else {
            panic!(
                "Expected command line args+opts \"{}\" to fail, but it passed instead!",
                args.join(" ")
            );
        }
    }

    #[test]
    fn test_clear() {
        let opts = expect_clear_opts(getopts(&["clear"]));
        assert![!opts.d()];
    }

    #[test]
    fn test_debug() {
        let opts = expect_wifi_connect_opts(getopts(&[]));
        assert![!opts.d()];
        let opts = expect_wifi_connect_opts(getopts(&["-d"]));
        assert![opts.d()];
        let opts = expect_wifi_connect_opts(getopts(&["-d", "wifi"]));
        assert![opts.d()];
        let opts = expect_wifi_connect_opts(getopts(&["-d", "wifi", "connect"]));
        assert![opts.d()];
        let opts = expect_wifi_connect_opts(getopts(&["--debug"]));
        assert![opts.d()];
    }

    #[test]
    fn test_interface() {
        let opts = expect_wifi_connect_opts(getopts(&["wifi", "-i", "BFUGG"]));
        assert_eq![opts.get_given_interface_name(), &Some("BFUGG".to_string())];
        let opts = expect_wifi_connect_opts(getopts(&["wifi", "--interface", "BLEEBLOO"]));
        assert_eq![opts.get_given_interface_name(), &Some("BLEEBLOO".to_string())];
    }

    #[test]
    fn test_ignore_known() {
        let opts = expect_wifi_connect_opts(getopts(&["wifi"]));
        assert![!opts.get_ignore_known()];
        let opts = expect_wifi_connect_opts(getopts(&["wifi", "--ignore-known"]));
        assert![opts.get_ignore_known()];
    }

    #[test]
    fn test_scan_type() {
        let wifi_type = WifiScanType::WpaCli;
        let opts =
            expect_wifi_connect_opts(getopts(&["wifi", "-s", wifi_type.to_string().as_ref()]));
        assert_eq![opts.get_scan_type(), &wifi_type];

        let wifi_type = WifiScanType::IW;
        let opts = expect_wifi_connect_opts(getopts(&[
            "wifi",
            "--scan-type",
            wifi_type.to_string().as_ref(),
        ]));
        assert_eq![opts.get_scan_type(), &wifi_type];
    }

    #[test]
    fn test_scan_method_default() {
        let scan_type = WifiScanType::default();
        let scan_method = ScanMethod::default();
        let opts = expect_wifi_connect_opts(getopts(&[]));
        assert_eq![opts.get_scan_method(), &scan_method];
        assert_eq![opts.get_scan_type(), &scan_type];
    }

    #[test]
    fn test_scan_method_stdin() {
        let wifi_scan_type = WifiScanType::default();
        let scan_method = ScanMethod::FromStdin;
        let opts = expect_wifi_connect_opts(getopts(&["wifi", "-I"]));
        assert_eq![opts.get_scan_method(), &scan_method];
        assert_eq![opts.get_scan_type(), &wifi_scan_type];

        let wifi_scan_type = WifiScanType::WpaCli;
        let scan_method = ScanMethod::FromStdin;
        let opts = expect_wifi_connect_opts(getopts(&[
            "wifi",
            "-I",
            "-s",
            wifi_scan_type.to_string().as_ref(),
        ]));
        assert_eq![opts.get_scan_method(), &scan_method];
        assert_eq![opts.get_scan_type(), &wifi_scan_type];
    }

    #[test]
    fn test_invalid_type_and_method() {
        let wst = WifiScanType::RuwiJSON;
        let opts = getopts_safe(&["wifi", "-s", wst.to_string().as_ref()]);
        assert![opts.is_err()];
    }

    #[test]
    fn test_selection_method() {
        let expected = SelectionMethod::Fzf;
        let opts = expect_wifi_connect_opts(getopts(&["-m", expected.to_string().as_ref()]));
        assert_eq![opts.get_selection_method(), &expected];

        let expected = SelectionMethod::Dmenu;
        let opts = expect_wifi_connect_opts(getopts(&[
            "--selection-method",
            expected.to_string().as_ref(),
        ]));
        assert_eq![opts.get_selection_method(), &expected];
    }

    #[test]
    fn test_give_password() {
        let pw = "fakepasswordddd";
        let opts = expect_wifi_connect_opts(getopts(&["wifi", "connect", "-p", pw]));
        assert_eq![opts.get_given_encryption_key().clone().unwrap(), pw];

        let pw2 = "FAKEP_SSS_A_W_W_W_W";
        let opts = expect_wifi_connect_opts(getopts(&["wifi", "connect", "--password", pw2]));
        assert_eq![opts.get_given_encryption_key().clone().unwrap(), pw2];
    }

    #[test]
    fn test_force_ask_password() {
        let opts = expect_wifi_connect_opts(getopts(&["wifi", "connect"]));
        assert![!opts.get_force_ask_password()];

        let opts = expect_wifi_connect_opts(getopts(&["wifi", "connect", "-P"]));
        assert![opts.get_force_ask_password()];

        let opts = expect_wifi_connect_opts(getopts(&["wifi", "connect", "--force-ask-password"]));
        assert![opts.get_force_ask_password()];
    }

    #[test]
    fn test_auto_mode() {
        let opts = expect_wifi_connect_opts(getopts(&["wifi", "connect"]));
        assert_eq![opts.get_auto_mode(), &AutoMode::default()];

        let opts = expect_wifi_connect_opts(getopts(&["wifi", "connect", "-a"]));
        assert_eq![opts.get_auto_mode(), &AutoMode::KnownOrAsk];

        let opts = expect_wifi_connect_opts(getopts(&["wifi", "connect", "--auto"]));
        assert_eq![opts.get_auto_mode(), &AutoMode::KnownOrAsk];

        let opts = expect_wifi_connect_opts(getopts(&[
            "wifi",
            "connect",
            "-A",
            AutoMode::KnownOrFail.to_string().as_ref(),
        ]));
        assert_eq![opts.get_auto_mode(), &AutoMode::KnownOrFail];

        let opts = expect_wifi_connect_opts(getopts(&[
            "wifi",
            "connect",
            "-A",
            AutoMode::First.to_string().as_ref(),
        ]));
        assert_eq![opts.get_auto_mode(), &AutoMode::First];

        let opts = expect_wifi_connect_opts(getopts(&[
            "wifi",
            "connect",
            "--auto-mode",
            AutoMode::KnownOrAsk.to_string().as_ref(),
        ]));
        assert_eq![opts.get_auto_mode(), &AutoMode::KnownOrAsk];
    }

    #[test]
    fn test_dry_run_in_tests() {
        let opts = expect_wifi_connect_opts(getopts(&[]));
        assert![!opts.get_dry_run()];
        let opts = expect_wifi_connect_opts(getopts(&["-D"]));
        assert![opts.get_dry_run()];
        let opts = expect_wifi_connect_opts(getopts(&["--dry-run"]));
        assert![opts.get_dry_run()];
    }

    #[test]
    fn test_force_synchronous_scan() {
        let opts = expect_wifi_connect_opts(getopts(&["wifi"]));
        assert![!opts.get_force_synchronous_scan()];

        let opts = expect_wifi_connect_opts(getopts(&["wifi", "-f"]));
        assert![opts.get_force_synchronous_scan()];

        let opts = expect_wifi_connect_opts(getopts(&["wifi", "--force-sync"]));
        assert![opts.get_force_synchronous_scan()];
    }

    #[test]
    fn test_incorrect_enum_variant_invocations() {
        test_fail_incorrect_option(&["wifi", "-s", "BOOOBLOOOBOO"]);
        test_fail_incorrect_option(&["wifi", "--selection-method", "BOOWOEOOOOOO"]);
        test_fail_incorrect_option(&["-s", "HARBLGARBL"]);
        test_fail_incorrect_option(&["--scan-type", "HARBLGARBL"]);
        test_fail_incorrect_option(&["-c", "BLAHBLAHBLAHWHOO"]);
        test_fail_incorrect_option(&["--connect-via", "YERAFKNWIZ"]);
    }

    #[test]
    fn test_fail_scan_type_and_method_validation() {
        let err = getopts_safe(&["wifi", "-s", "ruwi_json"]).unwrap_err();
        dbg!(&err);
        assert_eq![err.kind, RuwiErrorKind::InvalidScanTypeAndMethod];
    }

    #[test]
    fn test_invalid_scan_type_and_connect_type() {
        let err = getopts_safe(&["wifi", "-s", "iw", "connect", "-c", "nmcli"]).unwrap_err();
        dbg!(&err);
        assert_eq![err.kind, RuwiErrorKind::InvalidScanTypeAndConnectType];
    }


    #[test]
    fn test_wired_connect_via() {
        for connect_type in RawInterfaceConnectionType::iter() {
            dbg!(&connect_type);
            let opts = expect_wired_connect_opts(getopts(&[
                "wired",
                "connect",
                "--connect-via",
                &connect_type.to_string(),
            ]));
            assert_eq![opts.get_connect_via(), &connect_type];
        }
    }
}
