use crate::structs::*;
use std::cmp::Ordering;
use std::collections::HashSet;

impl Ord for AnnotatedWirelessNetwork {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.known ^ other.known {
            self.known.cmp(&other.known)
        } else {
            self.signal_strength.cmp(&other.signal_strength)
        }
    }
}

impl PartialOrd for AnnotatedWirelessNetwork {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) fn sort_and_filter_networks(
    options: &Options,
    annotated_networks: AnnotatedNetworks,
) -> SortedUniqueNetworks {
    let mut sorted_networks = annotated_networks.networks.clone();
    put_best_networks_first(&mut sorted_networks);

    // TODO: Once partition_dedup_by is stable:
    //let (sorted_unique_networks, _dups) = sorted_networks.partition_dedup_by(|a, b| a.essid == b.essid);
    let mut sorted_unique_networks = vec![];
    let mut seen_network_names = HashSet::new();
    for nw in sorted_networks {
        let essid = nw.essid.clone();
        if !seen_network_names.contains(&essid) {
            seen_network_names.insert(essid.clone());

            sorted_unique_networks.push(nw);
        }
    }

    if options.debug {
        dbg![&sorted_unique_networks];
    }

    SortedUniqueNetworks {
        networks: sorted_unique_networks,
    }
}

pub(crate) fn put_best_networks_first(networks: &mut Vec<AnnotatedWirelessNetwork>) {
    networks.sort();
    networks.reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_order(
        should_be_first: AnnotatedWirelessNetwork,
        should_be_last: AnnotatedWirelessNetwork,
    ) {
        let wrong_order = vec![should_be_last.clone(), should_be_first.clone()];

        let mut right_order = wrong_order;
        right_order.sort();

        assert_eq![right_order, vec![should_be_first, should_be_last]];
    }

    #[test]
    fn test_strength_sorting() {
        let higher_signal = AnnotatedWirelessNetwork {
            essid: "Valparaiso_Guest_House 1".to_string(),
            signal_strength: Some(-66),
            ..Default::default()
        };

        let lower_signal = AnnotatedWirelessNetwork {
            essid: "Valparaiso_Guest_House 1".to_string(),
            signal_strength: Some(-69),
            ..Default::default()
        };

        compare_order(lower_signal, higher_signal);
    }

    #[test]
    fn test_known_higher_than_unknown() {
        let known = AnnotatedWirelessNetwork {
            essid: "Valparaiso_Guest_House 1".to_string(),
            known: true,
            ..Default::default()
        };

        let not_known = AnnotatedWirelessNetwork {
            essid: "Valparaiso_Guest_House 1".to_string(),
            known: false,
            ..Default::default()
        };

        compare_order(not_known, known);
    }

    #[test]
    fn test_known_higher_than_unknown_with_higher_signal() {
        let known = AnnotatedWirelessNetwork {
            essid: "Valparaiso_Guest_House 1".to_string(),
            known: true,
            signal_strength: Some(-80),
            ..Default::default()
        };

        let not_known = AnnotatedWirelessNetwork {
            essid: "Valparaiso_Guest_House 1".to_string(),
            known: false,
            signal_strength: Some(-20),
            ..Default::default()
        };

        compare_order(not_known, known);
    }

    #[test]
    fn test_unique_nw_name_sort() {
        let networks = vec![
            AnnotatedWirelessNetwork {
                essid: "DOOK".to_string(),
                signal_strength: Some(-5),
                ..Default::default()
            },
            AnnotatedWirelessNetwork {
                essid: "BOYS".to_string(),
                signal_strength: Some(-47),
                ..Default::default()
            },
            AnnotatedWirelessNetwork {
                essid: "DOOK".to_string(),
                signal_strength: Some(-49),
                ..Default::default()
            },
            AnnotatedWirelessNetwork {
                essid: "YES".to_string(),
                signal_strength: Some(-89),
                ..Default::default()
            },
        ];

        let expected_networks = vec![
            networks[0].clone(),
            networks[1].clone(),
            networks[3].clone(),
        ];

        let sorted_unique_networks =
            sort_and_filter_networks(&Options::default(), AnnotatedNetworks { networks });

        assert_eq![expected_networks, sorted_unique_networks.networks];
    }
}
