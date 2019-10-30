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

fn get_val<T: FromStr + Default>(m: ArgMatches, arg: &str) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let scan_type = match m.value_of(arg) {
        Some(x) => T::from_str(x).expect(&format!("Failed to parse: {}", arg)),
        None => T::default(),
    };
    scan_type
}

fn get_matches<'a>() -> ArgMatches<'a> {
    let debug = Arg::with_name("debug")
        .short("d")
        .long("debug")
        .help("Print out debug information on what ruwi sees.");

    let list_networks = SubCommand::with_name("list_networks")
        .about("Scan and print out the visible SSIDs, with no additional information.");

    let scan_type = Arg::with_name("scan_type")
        .short("t")
        .long("scan_type")
        .default_value("wpa_cli")
        .possible_values(&possible_vals::<ScanType, _>())
        .help("The scanning method to use. Only wpa_cli is currently implemented.");

    App::new("Rust Wireless Manager")
        .version("0.2")
        .author("Glenn Hope <glenn.alexander.hope@gmail.com>")
        .about("Finds, selects, and configures new wireless networks.")
        .subcommand(list_networks)
        .arg(debug)
        .arg(scan_type)
        .get_matches()
}

pub fn get_options() -> Options {
    let m = get_matches();

    let debug = m.is_present("debug");

    let x = match m.value_of("scan_type") {
        Some(x) => ScanType::from_str(x).expect(&format!("Failed to parse {}", "scan_type")),
        None => ScanType::default(),
    };
    let scan_type = get_val::<ScanType>(m, "scan_type");

    let opts = Options {
        scan_type,
        selection_method: SelectionMethod::Dmenu,
        output_types: vec![OutputType::NetctlConfig],
        interface: "wlp3s0".to_string(),
        connect_via: Some(ConnectionType::Netctl),
        debug,
    };

    dbg!(&opts);

    if opts.debug {
        dbg!(&opts);
    }
    opts
}

// TODO: detect if run in interactive mode, and use fzf if so - dmenu otherwise
