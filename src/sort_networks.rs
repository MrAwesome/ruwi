use std::cmp::Ordering;

use crate::structs::*;

impl Ord for WirelessNetwork {
    fn cmp(&self, other: &Self) -> Ordering {
        self.signal_strength.cmp(&other.signal_strength)
    }
}

impl PartialOrd for WirelessNetwork {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) fn get_and_sort_available_networks(
    options: &Options,
    parse_results: &ParseResult,
) -> Vec<WirelessNetwork> {
    let mut sorted_networks = parse_results.seen_networks.clone();
    put_strongest_networks_first(&mut sorted_networks);

    if options.debug {
        dbg!(&sorted_networks);
    }

    sorted_networks
}

pub(crate) fn put_strongest_networks_first(networks: &mut Vec<WirelessNetwork>) {
    networks.sort();
    networks.reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_sorting() {
        let higher_signal = WirelessNetwork {
            essid: "Valparaiso_Guest_House 1".to_string(),
            is_encrypted: true,
            bssid: Some("f4:28:53:fe:a5:d0".to_string()),
            signal_strength: Some(-66),
            channel_utilisation: None,
        };

        let lower_signal = WirelessNetwork {
            essid: "Valparaiso_Guest_House 1".to_string(),
            is_encrypted: true,
            bssid: Some("f4:28:53:fe:a5:d0".to_string()),
            signal_strength: Some(-69),
            channel_utilisation: None,
        };

        let mut wrong_order = vec![higher_signal.clone(), lower_signal.clone()];

        wrong_order.sort();
        let right_order = wrong_order;

        assert_eq![right_order, vec![lower_signal, higher_signal]];
    }
}
