use crate::structs::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use strum::AsStaticRef;
use strum::IntoEnumIterator;

//fn get_all_vals<'a, T: Iterator<Item = AsStaticRef>>(lawl: T) -> &'a [&'static str] {
//fn get_all_vals<'a, T: AsStaticRef, I: Iterator<Item = T>>(iter: I) -> &'a [&'static str] {
//    //let lawl = ScanType::iter();
//    &iter.map(|x| x.as_static()).collect::<Vec<_>>()[..]
//}

fn lawl() {
    let lawl: Vec<&'static str> = possible_vals::<ScanType, _>();
    dbg!(lawl);
}

fn possible_vals<'a, E, I>() -> Vec<&'static str>
where
    E: IntoEnumIterator<Iterator = I> + AsStaticRef<str>,
    I: Iterator<Item = E>,
{
    E::iter().map(|x| x.as_static()).collect::<Vec<_>>()
}

fn get_matches<'a>() -> ArgMatches<'a> {
    lawl();
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

    let nm = m.value_of("scan_type").or(Some("wpa_cli"));
    //dbg!(ScanType::iter());

    //let lawl = ScanType::from_str(nm);
    //ScanType.default_value();
    //dbg!(lawl);
    let lawl = ScanType::iter().collect::<Vec<_>>();
    dbg!(lawl);

    let scan_type = ScanType::WpaCli;

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
