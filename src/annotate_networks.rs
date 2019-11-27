use crate::structs::*;

// TODO: make type for known network names
use std::collections::HashSet;

pub(crate) fn annotate_networks(
    _options: &Options,
    networks: &Vec<WirelessNetwork>,
    known_network_names: &HashSet<String>,
) -> AnnotatedNetworks {
    let networks = networks
        .iter()
        .map(|nw| WirelessNetwork {
            known: known_network_names.contains(&nw.essid),
            ..Default::default()
        })
        .collect();

    AnnotatedNetworks { networks }
}
