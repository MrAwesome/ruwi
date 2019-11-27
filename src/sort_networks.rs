use crate::structs::*;
use std::cmp::Ordering;

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

pub(crate) fn sort_available_networks(
    options: &Options,
    annotated_networks: AnnotatedNetworks,
) -> SortedNetworks {
    let mut networks = annotated_networks.networks;
    put_best_networks_first(&mut networks);

    if options.debug {
        dbg![&networks];
    }

    SortedNetworks { networks }
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
}
