use crate::get_default_interface::get_default_interface;
use crate::rerr;
use crate::structs::*;
use crate::strum_utils::{get_val_as_enum, possible_vals};
use clap::{App, Arg, ArgMatches};
use strum::AsStaticRef;

// TODO: fail if not run as root
// TODO: allow essid to be provided with -e XXX
// TODO: use subcommands for configurations of options, but still go through all functions always?
//       or just run certain functions for certain subcommands?
// TODO: arg for not connecting?
fn get_arg_app<'a, 'b>() -> App<'a, 'b> {
    // TODO: move these to be subcommand args for only the relevant subcommands
    let debug = Arg::with_name("debug")
        .short("d")
        .long("debug")
        .help("Print out debug information on what ruwi sees.");

    let auto = Arg::with_name("auto").short("a").long("auto").help(
        "Connect to the strongest known network seen. Will prompt for selection if no known networks are seen.",
    );

    let auto_no_ask = Arg::with_name("auto_no_ask")
        .short("A")
        .long("auto-no-ask")
        .help("When in auto mode, just fail if no know networks are seen. Takes precedence over `-a`.");

    let force_synchronous = Arg::with_name("force_synchronous_scan")
        .short("f")
        .long("force-sync")
        .help("Do not look at cached results, always scan for networks when run.");

    let interface = Arg::with_name("interface")
        .short("i")
        .long("interface")
        .takes_value(true)
        .help("The wireless device interface (e.g. wlp3s0) to use. Will attempt to use wpa_cli to infer it, if none given.");

    let essid = Arg::with_name("essid")
        .short("e")
        .long("essid")
        .takes_value(true)
        .help("Manually specify wireless network name (aka SSID or ESSID). Will be asked for if not given.");

    let password = Arg::with_name("password")
        .short("p")
        .long("password")
        .takes_value(true)
        .help("Manually specify encryption key (aka password). To read from a file, try \"$(cat your/file/name)\".");

    let scan_type = Arg::with_name("scan_type")
        .short("s")
        .long("scan-type")
        .default_value(&ScanType::default().as_static())
        .possible_values(&possible_vals::<ScanType, _>())
        .help("The wifi scanning program to use to get results.");

    let scan_method = Arg::with_name("scan_method")
        .short("S")
        .long("scan-method")
        .default_value(&ScanMethod::default().as_static())
        .possible_values(&possible_vals::<ScanMethod, _>())
        .help("How to get the scan output - by running the scan now, by reading from a file, or by reading from stdin. Unless you're a power user or writing tests, you can safely ignore this option.");

    let selection_method = Arg::with_name("selection_method")
        .short("m")
        .long("selection-method")
        .default_value(&SelectionMethod::default().as_static())
        .possible_values(&possible_vals::<SelectionMethod, _>())
        .help("The program to use to prompt for input.");

    let connect_via = Arg::with_name("connect_via")
        .short("c")
        .long("connect-via")
        .default_value(&ConnectionType::default().as_static())
        .possible_values(&possible_vals::<ConnectionType, _>())
        .help("Which network management suite to use to connect, or whether to just print the selected SSID for use elsewhere.");

    App::new("Rust Wireless Manager")
        .version("0.2")
        .author("Glenn Hope <glenn.alexander.hope@gmail.com>")
        .about("Combines existing network management layers (netctl, NetworkManager, wicd) and selection utilities (fzf, dmenu) to find, select, configure, and connect to wireless networks.")
        .arg(auto)
        .arg(auto_no_ask)
        .arg(connect_via)
        .arg(debug)
        .arg(essid)
        .arg(force_synchronous)
        .arg(interface)
        .arg(password)
        .arg(scan_type)
        .arg(scan_method)
        .arg(selection_method)
}

pub fn get_options() -> Result<Options, RuwiError> {
    let m = get_arg_app().get_matches();
    get_options_impl(m)
}

fn get_options_impl(m: ArgMatches) -> Result<Options, RuwiError> {
    let debug = m.is_present("debug");

    let auto_mode = if m.is_present("auto_no_ask") {
        AutoMode::AutoNoAsk
    } else if m.is_present("auto") {
        AutoMode::Auto
    } else {
        AutoMode::None
    };

    let force_synchronous_scan = m.is_present("force_synchronous_scan");

    let given_essid = m.value_of("essid").map(String::from);
    let given_encryption_key = m.value_of("password").map(String::from);
    let interface = match m.value_of("interface") {
        Some(val) => String::from(val),
        None => get_default_interface(debug)?,
    };

    let scan_type = get_val_as_enum::<ScanType>(&m, "scan_type");
    let scan_method = get_val_as_enum::<ScanMethod>(&m, "scan_method");

    validate_scan_type_and_method(&scan_type, &scan_method)?;

    let selection_method = get_val_as_enum::<SelectionMethod>(&m, "selection_method");
    let connect_via = get_val_as_enum::<ConnectionType>(&m, "connect_via");

    let opts = Options {
        scan_type,
        scan_method,
        selection_method,
        interface,
        connect_via,
        debug,
        given_essid,
        given_encryption_key,
        auto_mode,
        force_synchronous_scan,
        ..Default::default()
    };

    if opts.debug {
        dbg![&opts];
    }
    Ok(opts)
}

fn validate_scan_type_and_method(
    scan_type: &ScanType,
    scan_method: &ScanMethod,
) -> Result<(), RuwiError> {
    match (scan_type, scan_method) {
        (ScanType::RuwiJSON, ScanMethod::ByRunning) => Err(rerr!(
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
        get_options_impl(get_matches(args)).unwrap()
    }

    fn getopts_safe(args: &[&str]) -> Result<Options, RuwiError> {
        get_options_impl(get_matches_safe(args).map_err(|e| {
            rerr!(
                RuwiErrorKind::TestCmdLineOptParserSafeFailed,
                e.description()
            )
        })?)
    }

    #[test]
    fn test_debug() {
        let opts = getopts(&["--debug"]);
        assert![opts.debug];
    }

    #[test]
    fn test_interface() {
        let opts = getopts(&["--interface", "BLEEBLOOFAKEIFACE"]);
        assert_eq![opts.interface, "BLEEBLOOFAKEIFACE"];
    }

    #[test]
    fn test_scan_type() {
        let expected = ScanType::WpaCli.to_string();
        let opts = getopts(&["-s", expected.as_ref()]);
        assert_eq![opts.scan_type.to_string(), expected];

        let expected = ScanType::IW.to_string();
        let opts = getopts(&["--scan-type", expected.as_ref()]);
        assert_eq![opts.scan_type.to_string(), expected];
    }

    #[test]
    fn test_scan_method() {
        let expected = ScanMethod::ByRunning.to_string();
        let opts = getopts(&["-S", expected.as_ref()]);
        assert_eq![opts.scan_method.to_string(), expected];

        let expected = ScanMethod::FromFile.to_string();
        let opts = getopts(&["--scan-method", expected.as_ref()]);
        assert_eq![opts.scan_method.to_string(), expected];
    }

    #[test]
    fn test_invalid_type_and_method() {
        let st = ScanType::RuwiJSON.to_string();
        let sm = ScanMethod::ByRunning.to_string();
        let opts = getopts_safe(&["-s", st.as_ref(), "-S", sm.as_ref()]);
        assert![opts.is_err()];
    }

    #[test]
    fn test_selection_method() {
        let expected = SelectionMethod::Fzf.to_string();
        let opts = getopts(&["-m", expected.as_ref()]);
        assert_eq![opts.selection_method.to_string(), expected];

        let expected = SelectionMethod::Dmenu.to_string();
        let opts = getopts(&["--selection-method", expected.as_ref()]);
        assert_eq![opts.selection_method.to_string(), expected];
    }

    #[test]
    fn test_auto_mode() {
        let opts = getopts(&[]);
        assert_eq![opts.auto_mode, AutoMode::None];

        let opts = getopts(&["-a"]);
        assert_eq![opts.auto_mode, AutoMode::Auto];

        let opts = getopts(&["--auto"]);
        assert_eq![opts.auto_mode, AutoMode::Auto];

        let opts = getopts(&["-A"]);
        assert_eq![opts.auto_mode, AutoMode::AutoNoAsk];

        let opts = getopts(&["--auto-no-ask"]);
        assert_eq![opts.auto_mode, AutoMode::AutoNoAsk];
    }

    #[test]
    fn test_force_synchronous_scan() {
        let opts = getopts(&[]);
        assert![!opts.force_synchronous_scan];

        let opts = getopts(&["-f"]);
        assert![opts.force_synchronous_scan];

        let opts = getopts(&["--force-sync"]);
        assert![opts.force_synchronous_scan];
    }

    #[test]
    fn test_incorrect_selection_method() {
        let short_res = getopts_safe(&["-s", "BOOOBLOOOBOO"]);
        let long_res = getopts_safe(&["--selection-method", "BOOWOEOOOOOO"]);

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
