use crate::get_default_interface::get_default_interface;
use crate::rerr;
use crate::structs::*;
use crate::strum_utils::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use strum::AsStaticRef;

// TODO: respect force_ask_password
// TODO: fail if not run as root
// TODO: use subcommands for conuigurations of options, but still go through all functions always?
//       or just run certain functions for certain subcommands?
#[allow(clippy::too_many_lines)]
fn get_arg_app<'a, 'b>() -> App<'a, 'b> {
    // TODO: use .aliases for commands
    // TODO: set (.global(true)).subcommand() where needed?
    let debug = Arg::with_name("debug")
        .short("d")
        .long("debug")
        .global(true)
        .help("Print out debug information on what ruwi sees.");

    let dry_run = Arg::with_name("dry_run")
        .short("D")
        .long("dry-run")
        .global(true)
        .help("Don't write to or read from any files, interfaces, or networks. Mostly just useful for integration tests.");

    let input_file = Arg::with_name("input_file")
        .short("F")
        .long("input-file")
        .takes_value(true)
        .help("Instead of running a scan, use scan results from specified file.");

    let input_stdin = Arg::with_name("input_stdin")
        .short("I")
        .long("input-stdin")
        .global(true)
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
        .global(true)
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
        .global(true)
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
        .arg(input_file)
        .arg(input_stdin)
        .subcommand(
            SubCommand::with_name("wifi")
            .arg(ignore_known)
            .arg(force_synchronous)
            .arg(wifi_interface)
            .arg(wifi_scan_type)
            .subcommand(
                SubCommand::with_name("connect")
                    .arg(auto)
                    .arg(auto_mode)
                    .arg(connect_via)
                    .arg(essid)
                    .arg(force_ask_password)
                    .arg(password)
        ))
}

pub(crate) fn get_options() -> Result<Options, RuwiError> {
    let m = get_arg_app().get_matches();
    get_options_impl(&m)
}

fn get_options_impl(m: &ArgMatches) -> Result<Options, RuwiError> {
    let debug = m.is_present("debug");

    let force_synchronous_scan = m.is_present("force_synchronous_scan");
    let force_ask_password = m.is_present("force_ask_password");
    let ignore_known = m.is_present("ignore_known");

    let dry_run = m.is_present("dry_run");
    if dry_run {
        eprintln!("[NOTE] Running in dryrun mode! Will not run any external commands or write/read configs on disk, and will only use cached scan results.");
    }

    let given_essid = m.value_of("essid").map(String::from);
    let given_encryption_key = m.value_of("password").map(String::from);
    let interface = match m.value_of("interface") {
        Some(val) => String::from(val),
        None => get_default_interface(debug, dry_run)?,
    };

    // TODO: better structure where you grab all the subcommand-related options in one pass
    let TODO = "check for subcommand, and parse wifi-specific things there";
    let scan_type = match m.subcommand() {
        (subc_name, Some(sub_m)) if subc_name == RuwiCommand::Wifi(RuwiWifiCommand::default()).to_string() => {
            ScanType::Wifi(get_val_as_enum::<WifiScanType>(&m, "scan_type"))
        }
        // (subc_name, Some(sub_m)) if subc_name == RuwiCommand::Wired(Default::default()).to_string() => {}
        // (subc_name, Some(sub_m)) if subc_name == RuwiCommand::Bluetooth(Default::default()).to_string() => {}
        (_, Some(_)) => todo!("Non-wifi commands are not yet implemented."),
        (_, None) => ScanType::default(),
    };

    let scan_method = if let Some(filename) = m.value_of("input_file").map(String::from) {
        ScanMethod::FromFile(filename)
    } else if m.is_present("input_stdin") {
        ScanMethod::FromStdin
    } else {
        ScanMethod::ByRunning
    };
    validate_scan_method_and_type(&scan_method, &scan_type)?;

    dbg!(m.subcommand());

    let auto_mode = if m.is_present("auto") {
        AutoMode::KnownOrAsk
    } else {
        get_val_as_enum::<AutoMode>(&m, "auto_mode")
    };

    let selection_method = get_val_as_enum::<SelectionMethod>(&m, "selection_method");
    let connect_via = get_val_as_enum::<WifiConnectionType>(&m, "connect_via");

//    let TODO = "Clean up!";
//    let wifi_scan_type = get_val_as_enum::<WifiScanType>(&m, "scan_type");
//    let cmd_opts = CommandOptions::Wifi(WifiCommandOptions {
//        scan_type: wifi_scan_type,
//        scan_method,
//        interface,
//        ignore_known,
//        connect_via,
//        given_essid,
//        given_encryption_key,
//        auto_mode,
//        force_synchronous_scan,
//        force_ask_password,
//        ..Default::default()
//    });

    let TODO = "should options have the same structure as the cmdline options (WifiOptions, WifiConnectOptions) or similar?";
    let opts = Options {
        selection_method,
        debug,
        dry_run,
        scan_type,
        scan_method,
        interface,
        ignore_known,
        connect_via,
        given_essid,
        given_encryption_key,
        auto_mode,
        force_synchronous_scan,
        force_ask_password,
        ..Options::default()
    };

    if opts.debug {
        dbg![&opts];
    }
    Ok(opts)
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

    fn getopts(args: &[&str]) -> Options {
        get_options_impl(&get_matches(args)).unwrap()
    }

    fn getopts_safe(args: &[&str]) -> Result<Options, RuwiError> {
        get_options_impl(&get_matches_safe(args).map_err(|e| {
            rerr!(
                RuwiErrorKind::TestCmdLineOptParserSafeFailed,
                e.description()
            )
        })?)
    }

    #[test]
    fn test_debug() {
        let opts = getopts(&[]);
        assert![!opts.debug];
        let opts = getopts(&["-d"]);
        assert![opts.debug];
        let opts = getopts(&["--debug"]);
        assert![opts.debug];
    }

    #[test]
    fn test_interface() {
        let opts = getopts(&["wifi", "-i", "BFUGG"]);
        assert_eq![opts.interface, "BFUGG"];
        let opts = getopts(&["wifi", "connect", "-i", "HARBO"]);
        assert_eq![opts.interface, "HARBO"];
        let opts = getopts(&["wifi", "--interface", "BLEEBLOO"]);
        assert_eq![opts.interface, "BLEEBLOO"];
    }

    #[test]
    fn test_ignore_known() {
        let opts = getopts(&["wifi"]);
        assert![!opts.ignore_known];
        let opts = getopts(&["wifi", "connect"]);
        assert![!opts.ignore_known];
        let opts = getopts(&["wifi", "--ignore-known"]);
        assert![opts.ignore_known];
        let opts = getopts(&["wifi", "connect", "--ignore-known"]);
        assert![opts.ignore_known];
    }

    #[test]
    fn test_scan_type() {
        let wifi_type = WifiScanType::WpaCli;
        let expected = ScanType::Wifi(wifi_type.clone());
        let opts = getopts(&["wifi", "connect", "-s", wifi_type.to_string().as_ref()]);
        assert_eq![opts.scan_type, expected];

        let wifi_type = WifiScanType::IW;
        let expected = ScanType::Wifi(wifi_type.clone());
        let opts = getopts(&["wifi", "connect", "--scan-type", wifi_type.to_string().as_ref()]);
        assert_eq![opts.scan_type, expected];
    }

    #[test]
    fn test_scan_method_default() {
        let scan_type = ScanType::default();
        let scan_method = ScanMethod::default();
        let opts = getopts(&[]);
        assert_eq![opts.scan_method, scan_method];
        assert_eq![opts.scan_type, scan_type];
    }

    #[test]
    fn test_scan_method_stdin() {
        let scan_type = ScanType::default();
        let scan_method = ScanMethod::FromStdin;
        let opts = getopts(&["-I"]);
        assert_eq![opts.scan_method, scan_method];
        assert_eq![opts.scan_type, scan_type];

        let wifi_scan_type = WifiScanType::WpaCli;
        let scan_type = ScanType::Wifi(wifi_scan_type.clone());
        let scan_method = ScanMethod::FromStdin;
        let opts = getopts(&["wifi", "-I", "-s", wifi_scan_type.to_string().as_ref()]);
        assert_eq![opts.scan_method, scan_method];
        assert_eq![opts.scan_type, scan_type];
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
        let opts = getopts(&["-m", expected.to_string().as_ref()]);
        assert_eq![opts.selection_method, expected];

        let expected = SelectionMethod::Dmenu;
        let opts = getopts(&["--selection-method", expected.to_string().as_ref()]);
        assert_eq![opts.selection_method, expected];
    }

    #[test]
    fn test_give_password() {
        let pw = "fakepasswordddd";
        let opts = getopts(&["wifi", "connect", "-p", pw]);
        assert_eq![opts.given_encryption_key.unwrap(), pw];

        let pw2 = "FAKEP_SSS_A_W_W_W_W";
        let opts = getopts(&["wifi", "connect", "--password", pw2]);
        assert_eq![opts.given_encryption_key.unwrap(), pw2];
    }

    #[test]
    fn test_force_ask_password() {
        let opts = getopts(&["wifi", "connect"]);
        assert![!opts.force_ask_password];

        let opts = getopts(&["wifi", "connect", "-P"]);
        assert![opts.force_ask_password];

        let opts = getopts(&["wifi", "connect", "--force-ask-password"]);
        assert![opts.force_ask_password];
    }

    #[test]
    fn test_auto_mode() {
        let opts = getopts(&["wifi", "connect"]);
        assert_eq![opts.auto_mode, AutoMode::default()];

        let opts = getopts(&["wifi", "connect", "-a"]);
        assert_eq![opts.auto_mode, AutoMode::KnownOrAsk];

        let opts = getopts(&["wifi", "connect", "--auto"]);
        assert_eq![opts.auto_mode, AutoMode::KnownOrAsk];

        let opts = getopts(&[
            "wifi",
            "connect",
            "-A",
            AutoMode::KnownOrFail.to_string().as_ref(),
        ]);
        assert_eq![opts.auto_mode, AutoMode::KnownOrFail];

        let opts = getopts(&[
            "wifi",
            "connect",
            "-A",
            AutoMode::First.to_string().as_ref(),
        ]);
        assert_eq![opts.auto_mode, AutoMode::First];

        let opts = getopts(&[
            "wifi",
            "connect",
            "--auto-mode",
            AutoMode::KnownOrAsk.to_string().as_ref(),
        ]);
        assert_eq![opts.auto_mode, AutoMode::KnownOrAsk];
    }

    #[test]
    fn test_dry_run_in_tests() {
        let opts = getopts(&[]);
        assert![!opts.dry_run];
        let opts = getopts(&["-D"]);
        assert![opts.dry_run];
        let opts = getopts(&["--dry-run"]);
        assert![opts.dry_run];
    }

    #[test]
    fn test_force_synchronous_scan() {
        let opts = getopts(&["wifi", "connect"]);
        assert![!opts.force_synchronous_scan];

        let opts = getopts(&["wifi"]);
        assert![!opts.force_synchronous_scan];

        let opts = getopts(&["wifi", "-f"]);
        assert![opts.force_synchronous_scan];

        let opts = getopts(&["wifi", "connect", "-f"]);
        assert![opts.force_synchronous_scan];

        let opts = getopts(&["wifi", "connect", "--force-sync"]);
        assert![opts.force_synchronous_scan];
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
