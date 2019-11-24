use crate::structs::*;
use std::collections::HashSet;
use std::fs::{read_dir, DirEntry, File};
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub(crate) fn mark_known_networks(
    available_networks: &Vec<WirelessNetwork>,
    known_network_names: &HashSet<String>,
) -> Vec<WirelessNetwork> {
    available_networks
        .iter()
        .map(|nw| WirelessNetwork {
            known: known_network_names.contains(&nw.essid),
            ..nw.clone()
        })
        .collect::<Vec<_>>()
}

// TODO: parse escape chars in essid
pub(crate) fn find_known_network_names(options: &Options) -> io::Result<HashSet<String>> {
    let known_network_names = if options.output_type == OutputType::NetctlConfig
        || options.connect_via == ConnectionType::Netctl
    {
        find_known_netctl_networks()
    } else {
        Ok(HashSet::new())
    };

    if options.debug {
        dbg![&known_network_names];
    }

    known_network_names
}

fn find_known_netctl_networks() -> io::Result<HashSet<String>> {
    let netctl_path = Path::new("/etc/netctl");
    if netctl_path.is_dir() {
        // TODO: use tokio/etc to asynchronously read from these files
        let known_essids = read_dir(netctl_path)?
            .filter_map(|entry| get_essid_from_netctl_config_file(entry).ok())
            .filter_map(|x| x)
            .collect::<HashSet<String>>();

        Ok(known_essids)
    } else {
        Ok(HashSet::new())
    }
}

fn get_essid_from_netctl_config_file(entry: io::Result<DirEntry>) -> io::Result<Option<String>> {
    let entry = entry?;
    let path = entry.path();
    if path.is_file() {
        let mut contents = String::new();
        let mut f = File::open(&path)?;
        f.read_to_string(&mut contents)?;
        Ok(get_essid_from_netctl_config_text(contents))
    } else {
        Ok(None)
    }
}

fn get_essid_from_netctl_config_text(contents: String) -> Option<String> {
    contents
        .lines()
        .filter(|line| line.starts_with("ESSID="))
        .next()
        .map(|line| {
            line.trim_start_matches("ESSID=")
                .trim_start_matches('\'')
                .trim_start_matches('"')
                .trim_end_matches('\'')
                .trim_end_matches('"')
                .into()
        })
}
