use crate::prelude::*;

use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::Debug;

use crate::select::Selector;

#[derive(Debug)]
pub(crate) struct SortedFilteredNetworks<N: Debug> {
    networks: Vec<N>,
}

impl<N: AnnotatedRuwiNetwork> Selector<N> for SortedFilteredNetworks<N> {
    fn get_networks(&self) -> &[N] {
        &self.networks
    }

}

impl<N: Ord + Identifiable + Clone + Debug> SortedFilteredNetworks<N> {
    #[cfg(test)]
    pub(crate) fn get_networks_mut(&mut self) -> &mut [N] {
        &mut self.networks
    }

    pub(crate) fn new(networks: Vec<N>) -> Self {
        let mut networks = networks;
        Self::put_best_networks_first(&mut networks);
        let networks = Self::dedup_networks(networks);
        Self { networks }
    }

    fn put_best_networks_first(networks: &mut Vec<N>) {
        networks.sort();
        networks.reverse();
    }
}

impl<N: Identifiable + Clone + Debug> SortedFilteredNetworks<N> {
    fn dedup_networks(networks: Vec<N>) -> Vec<N> {
        // Once partition_dedup_by is stable:
        //let (unique_networks, _dups) = sorted_networks.partition_dedup_by(|a, b| a.essid == b.essid);
        let mut unique_networks = vec![];
        let mut seen_network_names = HashSet::new();
        for nw in networks {
            let identifier = nw.get_public_name();
            if !seen_network_names.contains(identifier) {
                seen_network_names.insert(identifier.to_string());

                unique_networks.push(nw.clone());
            }
        }
        unique_networks
    }
}

impl Ord for AnnotatedWirelessNetwork {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_known() ^ other.is_known() {
            self.is_known().cmp(&other.is_known())
        } else {
            self.get_signal_strength().cmp(&other.get_signal_strength())
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
        let higher_signal = AnnotatedWirelessNetwork::builder()
            .essid("Valparaiso_Guest_House 1".to_string())
            .signal_strength(Some(-66))
        .build();

        let lower_signal = AnnotatedWirelessNetwork::builder()
            .essid("Valparaiso_Guest_House 1".to_string())
            .signal_strength(Some(-69))
        .build();

        compare_order(lower_signal, higher_signal);
    }

    #[test]
    fn test_known_higher_than_unknown() {
        let known = AnnotatedWirelessNetwork::builder()
            .essid("Valparaiso_Guest_House 1".to_string())
            .service_identifier(NetworkServiceIdentifier::netctl_nw("some_id"))
        .build();

        let not_known = AnnotatedWirelessNetwork::builder()
            .essid("Valparaiso_Guest_House 1".to_string())
            .service_identifier(None)
        .build();

        compare_order(not_known, known);
    }

    #[test]
    fn test_known_higher_than_unknown_with_higher_signal() {
        let known = AnnotatedWirelessNetwork::builder()
            .essid("Valparaiso_Guest_House 1".to_string())
            .service_identifier(NetworkServiceIdentifier::netctl_nw("some_id"))
            .signal_strength(Some(-80))
        .build();

        let not_known = AnnotatedWirelessNetwork::builder()
            .essid("Valparaiso_Guest_House 1".to_string())
            .service_identifier(None)
            .signal_strength(Some(-20))
        .build();

        compare_order(not_known, known);
    }

    #[test]
    fn test_unique_nw_name_sort() {
        let networks = vec![
            AnnotatedWirelessNetwork::builder()
                .essid("DOOK".to_string())
                .signal_strength(Some(-5))
            .build(),
            AnnotatedWirelessNetwork::builder()
                .essid("BOYS".to_string())
                .signal_strength(Some(-47))
            .build(),
            AnnotatedWirelessNetwork::builder()
                .essid("DOOK".to_string())
                .signal_strength(Some(-49))
            .build(),
            AnnotatedWirelessNetwork::builder()
                .essid("YES".to_string())
                .signal_strength(Some(-89))
            .build(),
        ];

        let expected_networks = vec![
            networks[0].clone(),
            networks[1].clone(),
            networks[3].clone(),
        ];

        let sorted_unique_networks = SortedFilteredNetworks::new(networks);
        assert_eq![expected_networks, sorted_unique_networks.get_networks()];
    }
}
