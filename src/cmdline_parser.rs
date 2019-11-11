use crate::get_default_interface::get_default_interface;
use crate::structs::*;
use crate::strum_utils::{get_val, possible_vals};
use clap::{App, Arg};
use std::io;
use strum::AsStaticRef;

// TODO: fail if not run as root
// TODO: allow essid to be provided with -e XXX
// TODO: use subcommands for configurations of options, but still go through all functions always?
//       or just run certain functions for certain subcommands?
// TODO: detect if run in interactive terminal mode, and use fzf if so - dmenu otherwise
// TODO: arg for not connecting?
fn get_arg_app<'a, 'b>() -> App<'a, 'b> {
    // TODO: move these to be subcommand args for only the relevant subcommands
    let debug = Arg::with_name("debug")
        .short("d")
        .long("debug")
        .help("Print out debug information on what ruwi sees.");

    // Global Args
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
        .short("t")
        .long("scan-type")
        .default_value(&ScanType::default().as_static())
        .possible_values(&possible_vals::<ScanType, _>())
        .help("The wifi scanning program to use under the hood. If none given, will be inferred using wpa_cli.");

    let selection_method = Arg::with_name("selection_method")
        .short("s")
        .long("selection-method")
        .default_value(&SelectionMethod::default().as_static())
        .possible_values(&possible_vals::<SelectionMethod, _>())
        .help("The program to use to prompt for input.");

    let output_type = Arg::with_name("output_type")
        .short("o")
        .long("output-type")
        .default_value(&OutputType::default().as_static())
        .possible_values(&possible_vals::<OutputType, _>())
        .help("The program to use to prompt for input.");

    let connect_via = Arg::with_name("connect_via")
        .short("c")
        .long("connect-via")
        .default_value(&ConnectionType::default().as_static())
        .possible_values(&possible_vals::<ConnectionType, _>())
        .help("Which network management suite to use to connect.");

    App::new("Rust Wireless Manager")
        .version("0.2")
        .author("Glenn Hope <glenn.alexander.hope@gmail.com>")
        .about("Finds, selects, configures, and connects to new wireless networks.")
        .arg(connect_via)
        .arg(debug)
        .arg(interface)
        .arg(output_type)
        .arg(scan_type)
        .arg(selection_method)
        .arg(essid)
        .arg(password)
}

pub fn get_options() -> io::Result<Options> {
    let m = get_arg_app().get_matches();

    let debug = m.is_present("debug");

    let interface = match m.value_of("interface") {
        Some(val) => String::from(val),
        None => get_default_interface(debug)?,
    };

    let given_essid = m.value_of("essid").map(String::from);
    let given_password = m.value_of("password").map(String::from);

    let scan_type = get_val::<ScanType>(&m, "scan_type");
    let selection_method = get_val::<SelectionMethod>(&m, "selection_method");
    let output_type = get_val::<OutputType>(&m, "output_type");
    let connect_via = get_val::<ConnectionType>(&m, "connect_via");

    let opts = Options {
        scan_type,
        selection_method,
        output_type,
        interface,
        connect_via,
        debug,
        given_essid,
        given_password,
    };

    if opts.debug {
        dbg!(&opts);
    }
    Ok(opts)
}

// It would be better if these tests just checked the Options returned,
// but that requires either a clunky rewrite or losing the nice auto-exit
// functionality that clap provides by default
#[cfg(test)]
mod tests {
    use super::*;
    use clap::ArgMatches;

    fn gmf<'a>(args: &[&str]) -> ArgMatches<'a> {
        get_arg_app().get_matches_from(args)
    }

    fn gmf_safe<'a>(args: &[&str]) -> Result<ArgMatches<'a>, clap::Error> {
        get_arg_app().get_matches_from_safe(args)
    }

    #[test]
    fn test_debug() {
        let m = gmf(&["lol", "--debug"]);
        assert![m.is_present("debug")];
    }

    #[test]
    fn test_interface() {
        let m = gmf(&["lol", "--interface", "fake_interface"]);
        assert_eq![m.value_of("interface").unwrap(), "fake_interface"];
    }

    #[test]
    fn test_scan_type_long() {
        let expected = ScanType::IW.to_string();
        let m = gmf(&["lol", "--scan-type", expected.as_ref()]);
        assert_eq![m.value_of("scan_type").unwrap(), expected];
    }

    #[test]
    fn test_scan_type_short() {
        let expected = ScanType::IWList.to_string();
        let m = gmf(&["lol", "-t", expected.as_ref()]);
        assert_eq![m.value_of("scan_type").unwrap(), expected];
    }

    #[test]
    fn test_selection_method_long() {
        let expected = SelectionMethod::Dmenu.to_string();
        let m = gmf(&["lol", "--selection-method", expected.as_ref()]);
        assert_eq![m.value_of("selection_method").unwrap(), expected];
    }

    #[test]
    fn test_output_type_short() {
        let expected = OutputType::NetctlConfig.to_string();
        let m = gmf(&["lol", "-o", expected.as_ref()]);
        assert_eq![m.value_of("output_type").unwrap(), expected];
    }

    #[test]
    fn test_output_type_long() {
        let expected = OutputType::NetworkManagerConfig.to_string();
        let m = gmf(&["lol", "--output-type", expected.as_ref()]);
        assert_eq![m.value_of("output_type").unwrap(), expected];
    }

    #[test]
    fn test_selection_method_short() {
        let expected = SelectionMethod::Fzf.to_string();
        let m = gmf(&["lol", "-s", expected.as_ref()]);
        assert_eq![m.value_of("selection_method").unwrap(), expected];
    }

    #[test]
    fn test_incorrect() {
        let short_res = gmf_safe(&["lol", "-s", "JLDKJSJKLDSJ"]);
        let long_res = gmf_safe(&["lol", "--selection-method", "JLDKJSJKLDSJ"]);
        let short_failed = short_res.is_err();
        let long_failed = long_res.is_err();
        assert![short_failed];
        assert![long_failed];
    }
}
