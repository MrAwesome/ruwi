use crate::structs::*;

// TODO: make type for known network names
use std::collections::HashSet;

pub(crate) fn annotate_networks(
    options: &Options,
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

    if options.debug {
        dbg![&networks];
    }

    AnnotatedNetworks { networks }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_network_not_known() {
        assert![!WirelessNetwork::default().known];
    }

    #[test]
    fn test_annotate_known() {
        let essid = "hahahahahahahahaha".to_string();
        let nw = WirelessNetwork {
            essid: essid.clone(),
            ..Default::default()
        };
        let mut known_networks = HashSet::new();
        known_networks.insert(essid);
        let annotated_networks = annotate_networks(&Options::default(), &vec![nw], &known_networks);

        let resulting_nw = annotated_networks.networks.first().unwrap();
        assert![resulting_nw.known];
    }

    #[test]
    fn test_do_not_annotate_unknown() {
        let essid = "wheeeeeeeeeeeeeeee".to_string();
        let nw = WirelessNetwork {
            essid,
            ..Default::default()
        };
        let known_networks = Default::default();
        let annotated_networks = annotate_networks(&Options::default(), &vec![nw], &known_networks);

        let resulting_nw = annotated_networks.networks.first().unwrap();
        assert![!resulting_nw.known];
    }
}
