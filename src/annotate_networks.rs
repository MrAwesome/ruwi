use crate::find_known_network_names::find_known_network_names;
use crate::structs::*;

fn annotated_networks(
    options: &Options,
    networks: &Vec<WirelessNetwork>,
    known_network_names: &Vec<String>,
) -> AnnotatedNetworks {
    mark_known_networks(networks, known_network_names)
}
