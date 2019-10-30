use crate::structs::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::str::FromStr;
use strum::AsStaticRef;
use strum::IntoEnumIterator;

fn possible_vals<'a, E, I>() -> Vec<&'static str>
where
    E: IntoEnumIterator<Iterator = I> + AsStaticRef<str>,
    I: Iterator<Item = E>,
{
    E::iter().map(|x| x.as_static()).collect::<Vec<_>>()
}

fn get_val<T: FromStr + Default>(m: &ArgMatches, arg: &str) -> T
where
    T::Err: std::fmt::Debug,
{
    let scan_type = match m.value_of(arg) {
        Some(x) => T::from_str(x).expect(&format!("Failed to parse: {}", arg)),
        None => T::default(),
    };
    scan_type
}

// TODO: Determine this from wpa_cli interface or similar
fn get_default_interface<'a>() -> &'a str {
    "wlp3s0"
}

fn get_matches<'a>() -> ArgMatches<'a> {
    let debug = Arg::with_name("debug")
        .short("d")
        .long("debug")
        .help("Print out debug information on what ruwi sees.");

    let list_networks = SubCommand::with_name("list_networks")
        .about("Scan and print out the visible SSIDs, with no additional information.");

    let interface = Arg::with_name("interface")
        .short("i")
        .long("interface")
        .default_value(get_default_interface())
        .help("The wifi scanning program to use under the hood.");

    let scan_type = Arg::with_name("scan_type")
        .short("t")
        .long("scan_type")
        .default_value(&ScanType::default().as_static())
        .possible_values(&possible_vals::<ScanType, _>())
        .help("The wifi scanning program to use under the hood.");

    let selection_method = Arg::with_name("selection_method")
        .short("s")
        .long("selection_method")
        .default_value(&SelectionMethod::default().as_static())
        .possible_values(&possible_vals::<SelectionMethod, _>())
        .help("The program to use to prompt for input.");

    let output_type = Arg::with_name("output_type")
        .short("o")
        .long("output_type")
        .default_value(&OutputType::default().as_static())
        .possible_values(&possible_vals::<OutputType, _>())
        .help("The program to use to prompt for input.");

    let connect_via = Arg::with_name("connect_via")
        .short("c")
        .long("connect_via")
        .default_value(&ConnectionType::default().as_static())
        .possible_values(&possible_vals::<ConnectionType, _>())
        .help("Which network management suite to use to connect.");

    App::new("Rust Wireless Manager")
        .version("0.2")
        .author("Glenn Hope <glenn.alexander.hope@gmail.com>")
        .about("Finds, selects, and configures new wireless networks.")
        .subcommand(list_networks)
        .arg(connect_via)
        .arg(debug)
        .arg(interface)
        .arg(output_type)
        .arg(scan_type)
        .arg(selection_method)
        .get_matches()
}

pub fn get_options() -> Options {
    let m = get_matches();

    let debug = m.is_present("debug");
    let interface = m
        .value_of("interface")
        .unwrap_or_else(|| get_default_interface())
        .to_string();

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
    };

    if opts.debug {
        dbg!(&opts);
    }
    opts
}

// TODO: detect if run in interactive mode, and use fzf if so - dmenu otherwise
