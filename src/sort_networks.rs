use crate::options::interfaces::*;
use crate::structs::*;
use std::cmp::Ordering;
use std::collections::HashSet;

pub(crate) struct SortedFilteredNetworks<T> {
    networks: Vec<T>,
}

impl<T> SortedFilteredNetworks<T> {
    pub(crate) fn get_networks(&self) -> &[T] {
        &self.networks
    }

    #[cfg(test)]
    pub(crate) fn get_networks_mut(&mut self) -> &mut [T] {
        &mut self.networks
    }
}

impl<T: Ord + Identifiable + Clone> SortedFilteredNetworks<T> {
    pub(crate) fn new(networks: &[T]) -> Self {
        let mut networks = networks.clone().to_vec(); 
        Self::put_best_networks_first(&mut networks);
        let networks = Self::dedup_networks(networks);
        Self { networks }
    }

    fn put_best_networks_first(networks: &mut Vec<T>) {
        networks.sort();
        networks.reverse();
    }
}

impl<T: Identifiable + Clone> SortedFilteredNetworks<T> {
    fn dedup_networks(networks: Vec<T>) -> Vec<T> {
        // Once partition_dedup_by is stable:
        //let (unique_networks, _dups) = sorted_networks.partition_dedup_by(|a, b| a.essid == b.essid);
        let mut unique_networks = vec![];
        let mut seen_network_names = HashSet::new();
        for nw in networks {
            let identifier = nw.get_identifier();
            if !seen_network_names.contains(identifier) {
                seen_network_names.insert(identifier.to_owned());

                unique_networks.push(nw.clone());
            }
        }
        unique_networks
    }
}

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
            ..AnnotatedWirelessNetwork::default()
        };

        let lower_signal = AnnotatedWirelessNetwork {
            essid: "Valparaiso_Guest_House 1".to_string(),
            signal_strength: Some(-69),
            ..AnnotatedWirelessNetwork::default()
        };

        compare_order(lower_signal, higher_signal);
    }

    #[test]
    fn test_known_higher_than_unknown() {
        let known = AnnotatedWirelessNetwork {
            essid: "Valparaiso_Guest_House 1".to_string(),
            known: true,
            ..AnnotatedWirelessNetwork::default()
        };

        let not_known = AnnotatedWirelessNetwork {
            essid: "Valparaiso_Guest_House 1".to_string(),
            known: false,
            ..AnnotatedWirelessNetwork::default()
        };

        compare_order(not_known, known);
    }

    #[test]
    fn test_known_higher_than_unknown_with_higher_signal() {
        let known = AnnotatedWirelessNetwork {
            essid: "Valparaiso_Guest_House 1".to_string(),
            known: true,
            signal_strength: Some(-80),
            ..AnnotatedWirelessNetwork::default()
        };

        let not_known = AnnotatedWirelessNetwork {
            essid: "Valparaiso_Guest_House 1".to_string(),
            known: false,
            signal_strength: Some(-20),
            ..AnnotatedWirelessNetwork::default()
        };

        compare_order(not_known, known);
    }

    #[test]
    fn test_unique_nw_name_sort() {
        let networks = vec![
            AnnotatedWirelessNetwork {
                essid: "DOOK".to_string(),
                signal_strength: Some(-5),
                ..AnnotatedWirelessNetwork::default()
            },
            AnnotatedWirelessNetwork {
                essid: "BOYS".to_string(),
                signal_strength: Some(-47),
                ..AnnotatedWirelessNetwork::default()
            },
            AnnotatedWirelessNetwork {
                essid: "DOOK".to_string(),
                signal_strength: Some(-49),
                ..AnnotatedWirelessNetwork::default()
            },
            AnnotatedWirelessNetwork {
                essid: "YES".to_string(),
                signal_strength: Some(-89),
                ..AnnotatedWirelessNetwork::default()
            },
        ];

        let expected_networks = vec![
            networks[0].clone(),
            networks[1].clone(),
            networks[3].clone(),
        ];

        let sorted_unique_networks = SortedFilteredNetworks::new(&networks);
        assert_eq![expected_networks, sorted_unique_networks.get_networks()];
    }
}
