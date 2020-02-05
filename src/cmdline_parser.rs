use crate::get_default_interface::get_default_interface;
use crate::options::interfaces::*;
use crate::options::*;
use crate::options::wifi::*;
use crate::options::wifi::connect::WifiConnectOptions;
use crate::options::wifi::select::WifiSelectOptions;
use crate::rerr;
use crate::errors::*;
use crate::structs::*;
use crate::strum_utils::*;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::fmt::Debug;
use strum::AsStaticRef;


// TODO: respect force_ask_password
// TODO: fail if not run as root
// TODO: use subcommands for conuigurations of options, but still go through all functions always?
//       or just run certain functions for certain subcommands?
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
        .possible_values(&possible_vals::<AutoMode, _>())
        .help("The auto mode to use.");

    let force_synchronous = Arg::with_name("force_synchronous_scan")
        .short("f")
        .long("force-sync")
        .help("Do not look at cached results, always scan for networks when run.");

    let ignore_known = Arg::with_name("ignore_known")
        .long("ignore-known")
        .help("Do not try to determine if networks are already known. Passwords will be requested always in this mode.");

    let wifi_interface = Arg::with_name("interface")
        .short("i")
        .long("interface")
        .takes_value(true)
        .help("The wireless device interface (e.g. wlp3s0) to use. Will attempt to use wpa_cli to infer it, if none given.");

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
        .default_value(&WifiScanType::default().as_static())
        .possible_values(&possible_vals::<WifiScanType, _>())
        .help("The wifi scanning program to use to get results.");

    let selection_method = Arg::with_name("selection_method")
        .short("m")
        .long("selection-method")
        .takes_value(true)
        .default_value(&SelectionMethod::default().as_static())
        .possible_values(&possible_vals::<SelectionMethod, _>())
        .help("The program to use to prompt for input.");

    let connect_via = Arg::with_name("connect_via")
        .short("c")
        .long("connect-via")
        .takes_value(true)
        .default_value(&WifiConnectionType::default().as_static())
        .possible_values(&possible_vals::<WifiConnectionType, _>())
        .help("Which network management suite to use to connect, or whether to just print the selected SSID for use elsewhere.");

    App::new("Ruwi")
        .version("0.2")
        .author("Glenn Hope <glenn.alexander.hope@gmail.com>")
        .about("Combines existing network management layers (netctl, NetworkManager, wicd) and selection utilities (fzf, dmenu) to find, select, configure, and connect to wireless networks.")
        .arg(debug)
        .arg(dry_run)
        .arg(selection_method)
        .subcommand(SubCommand::with_name("wifi")
            .arg(ignore_known)
            .arg(input_file)
            .arg(input_stdin)
            .arg(force_synchronous)
            .arg(wifi_interface)
            .arg(wifi_scan_type)
            .subcommand(SubCommand::with_name("connect")
                .arg(auto.clone())
                .arg(auto_mode.clone())
                .arg(connect_via)
                .arg(essid)
                .arg(force_ask_password)
                .arg(password))
            .subcommand(SubCommand::with_name("select")
                .arg(auto)
                .arg(auto_mode)
            )
        )
}

pub(crate) fn get_command() -> Result<RuwiCommand, RuwiError> {

    let m = get_arg_app().get_matches();
    get_command_impl(&m)
}

// TODO: return an enum of options/commands types
fn get_command_impl(m: &ArgMatches) -> Result<RuwiCommand, RuwiError> {
    let debug = m.is_present("debug");
    let selection_method = get_val_as_enum::<SelectionMethod>(&m, "selection_method");

    let dry_run = m.is_present("dry_run");
    if dry_run {
        // TODO: actually use cached results, or remove that from the message here.
        eprintln!("[NOTE] Running in dryrun mode! Will not run any external commands (besides the requested prompt command) or write/read configs on disk, and will only use cached scan results.");
    }

    let globals = GlobalOptions::builder()
        .debug(debug)
        .dry_run(dry_run)
        .selection_method(selection_method)
        .build();

    let opts = match m.subcommand() {
        (subc_name, maybe_sub_m)
            if subc_name == RuwiCommand::Wifi(RuwiWifiCommand::default()).to_string()
                || subc_name == "" =>
        {
            RuwiCommand::Wifi(get_wifi_cmd(globals, maybe_sub_m)?)
        }

        // (subc_name, Some(sub_m)) if subc_name == RuwiCommand::Wired(Default::default()).to_string() => {}
        //    RuwiCommand::Wired(get_wired_cmd(globals, maybe_sub_m)?)
        // (subc_name, Some(sub_m)) if subc_name == RuwiCommand::Bluetooth(Default::default()).to_string() => {}
        (subc_name, _) => {
            dbg!(subc_name);
            todo!("Non-wifi commands are not yet implemented.");
        }
    };

    if debug {
        dbg![&opts];
    }
    Ok(opts)
}

fn get_wifi_cmd(
    globals: GlobalOptions,
    maybe_sub_m: Option<&ArgMatches>,
) -> Result<RuwiWifiCommand, RuwiError> {
    let sub_m = if let Some(sub_m) = maybe_sub_m {
        sub_m
    } else {
        return Ok(RuwiWifiCommand::Connect(
            WifiConnectOptions::builder()
                .wifi(
                    WifiOptions::builder()
                        // TODO: this feels like a nasty hack, and doesn't belong here? Just store
                        // None if none given, and calculate the default later? It is nice to have
                        // the default show up in --help, but it still feels off.
                        .interface(get_default_interface(&globals)?)
                        .build(),
                )
                .globals(globals)
                .build(),
        ));
    };

    match sub_m.subcommand() {
        (subc_name, _) if subc_name == "" => Ok(RuwiWifiCommand::Connect(
            WifiConnectOptions::builder()
                .wifi(get_wifi_opts_impl(&globals, sub_m, None)?)
                .globals(globals)
                .build(),
        )),
        // TODO: ew @ this string thing
        (subc_name, Some(sub_sub_m))
            if subc_name
                == RuwiWifiCommand::Connect(WifiConnectOptions::builder().build()).to_string() =>
        {
            let wifi_opts = 
                get_wifi_opts_impl(&globals, sub_m, Some(sub_sub_m))?;
            Ok(RuwiWifiCommand::Connect(get_wifi_connect_opts(
                globals,
                wifi_opts,
                sub_sub_m,
            )?))
        }

        (subc_name, Some(sub_sub_m))
            if subc_name
                == RuwiWifiCommand::Select(WifiSelectOptions::builder().build()).to_string() =>
        {
            let wifi_opts = 
                get_wifi_opts_impl(&globals, sub_m, Some(sub_sub_m))?;
            Ok(RuwiWifiCommand::Select(get_wifi_select_opts(
                globals,
                wifi_opts,
                sub_sub_m,
            )?))
        }

        _ => todo!("other command types"),
    }
}

fn get_wifi_connect_opts(
    globals: GlobalOptions,
    wifi_opts: WifiOptions,
    sub_sub_m: &ArgMatches,
) -> Result<WifiConnectOptions, RuwiError> {
    let force_ask_password = sub_sub_m.is_present("force_ask_password");
    let given_essid = sub_sub_m.value_of("essid").map(String::from);
    let given_encryption_key = sub_sub_m.value_of("password").map(String::from);

    let auto_mode = if sub_sub_m.is_present("auto") {
        AutoMode::KnownOrAsk
    } else {
        get_val_as_enum::<AutoMode>(&sub_sub_m, "auto_mode")
    };

    let connect_via = get_val_as_enum::<WifiConnectionType>(&sub_sub_m, "connect_via");

    let opts = WifiConnectOptions::builder()
        .globals(globals)
        .wifi(wifi_opts)
        .connect_via(connect_via)
        .given_essid(given_essid)
        .given_encryption_key(given_encryption_key)
        .auto_mode(auto_mode)
        .force_ask_password(force_ask_password)
        .build();
    Ok(opts)
}


fn get_wifi_select_opts(
    globals: GlobalOptions,
    wifi_opts: WifiOptions,
    sub_sub_m: &ArgMatches,
) -> Result<WifiSelectOptions, RuwiError> {
    let auto_mode = if sub_sub_m.is_present("auto") {
        AutoMode::KnownOrAsk
    } else {
        get_val_as_enum::<AutoMode>(&sub_sub_m, "auto_mode")
    };

    let opts = WifiSelectOptions::builder()
        .globals(globals)
        .wifi(wifi_opts)
        .auto_mode(auto_mode)
        .build();
    Ok(opts)
}

fn get_wifi_opts_impl(
    globals: &GlobalOptions,
    sub_m: &ArgMatches,
    _sub_sub_m: Option<&ArgMatches>,
) -> Result<WifiOptions, RuwiError> {
    let scan_method = get_scan_method(sub_m);
    let force_synchronous_scan = sub_m.is_present("force_synchronous_scan");
    let ignore_known = sub_m.is_present("ignore_known");
    let interface = get_wifi_interface(sub_m, globals)?;
    let scan_type = ScanType::Wifi(get_val_as_enum::<WifiScanType>(&sub_m, "scan_type"));
    validate_scan_method_and_type(&scan_method, &scan_type)?;

    let wifi_opts = WifiOptions::builder()
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

fn get_wifi_interface<O>(m: &ArgMatches, opts: &O) -> Result<String, RuwiError>
where
    O: Global + Debug,
{
    Ok(match m.value_of("interface") {
        Some(val) => String::from(val),
        None => get_default_interface(opts)?,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rerr;
    use clap::ArgMatches;
    use std::error::Error;

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

    // TODO: fix to return something more generic, aka ruwicommand
    fn getopts(args: &[&str]) -> RuwiCommand {
        get_command_impl(&get_matches(args)).unwrap()
    }

    fn expect_wifi_connect_opts(cmd: RuwiCommand) -> WifiConnectOptions {
        if let RuwiCommand::Wifi(RuwiWifiCommand::Connect(opts)) = cmd {
            opts
        } else {
            panic!("Expected command to be wifi connect, but got: {:?}", cmd);
        }
    }

    fn getopts_safe(args: &[&str]) -> Result<RuwiCommand, RuwiError> {
        get_command_impl(&get_matches_safe(args).map_err(|e| {
            rerr!(
                RuwiErrorKind::TestCmdLineOptParserSafeFailed,
                e.description()
            )
        })?)
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
        assert_eq![opts.get_interface(), "BFUGG"];
        let opts = expect_wifi_connect_opts(getopts(&["wifi", "--interface", "BLEEBLOO"]));
        assert_eq![opts.get_interface(), "BLEEBLOO"];
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
        let expected = ScanType::Wifi(wifi_type.clone());
        let opts = expect_wifi_connect_opts(getopts(&["wifi", "-s", wifi_type.to_string().as_ref()]));
        assert_eq![opts.get_scan_type(), &expected];

        let wifi_type = WifiScanType::IW;
        let expected = ScanType::Wifi(wifi_type.clone());
        let opts = expect_wifi_connect_opts(getopts(&["wifi", "--scan-type", wifi_type.to_string().as_ref()]));
        assert_eq![opts.get_scan_type(), &expected];
    }

    #[test]
    fn test_scan_method_default() {
        let scan_type = ScanType::default();
        let scan_method = ScanMethod::default();
        let opts = expect_wifi_connect_opts(getopts(&[]));
        assert_eq![opts.get_scan_method(), &scan_method];
        assert_eq![opts.get_scan_type(), &scan_type];
    }

    #[test]
    fn test_scan_method_stdin() {
        let scan_type = ScanType::default();
        let scan_method = ScanMethod::FromStdin;
        let opts = expect_wifi_connect_opts(getopts(&["wifi", "-I"]));
        assert_eq![opts.get_scan_method(), &scan_method];
        assert_eq![opts.get_scan_type(), &scan_type];

        let wifi_scan_type = WifiScanType::WpaCli;
        let scan_type = ScanType::Wifi(wifi_scan_type.clone());
        let scan_method = ScanMethod::FromStdin;
        let opts = expect_wifi_connect_opts(getopts(&["wifi", "-I", "-s", wifi_scan_type.to_string().as_ref()]));
        assert_eq![opts.get_scan_method(), &scan_method];
        assert_eq![opts.get_scan_type(), &scan_type];
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
        let opts = expect_wifi_connect_opts(getopts(&["--selection-method", expected.to_string().as_ref()]));
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
    fn test_incorrect_selection_method() {
        let short_res = getopts_safe(&["wifi", "-s", "BOOOBLOOOBOO"]);
        let long_res = getopts_safe(&["wifi", "--selection-method", "BOOWOEOOOOOO"]);

        let short_failed = short_res.is_err();
        let long_failed = long_res.is_err();
        assert![short_failed];
        assert![long_failed];
    }

    #[test]
    fn test_incorrect_scan_type() {
        let short_res = getopts_safe(&["-t", "HARBLGARBL"]);
        let long_res = getopts_safe(&["--scan-type", "HARBLGARBL"]);

        let short_failed = short_res.is_err();
        let long_failed = long_res.is_err();
        assert![short_failed];
        assert![long_failed];
    }

    #[test]
    fn test_incorrect_connection_method() {
        let short_res = getopts_safe(&["-c", "BLAHBLAHBLAHWHOO"]);
        let long_res = getopts_safe(&["--connect-via", "YERAFKNWIZ"]);

        let short_failed = short_res.is_err();
        let long_failed = long_res.is_err();
        assert![short_failed];
        assert![long_failed];
    }
}
