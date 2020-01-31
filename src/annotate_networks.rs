use crate::options::interfaces::Global;
use crate::structs::*;

// TODO: unit test that networks passed in equal networks passed out
pub(crate) fn annotate_networks<O>(
    options: &O,
    networks: &[WirelessNetwork],
    known_network_names: &KnownNetworkNames,
) -> AnnotatedNetworks
where
    O: Global,
{
    let networks = networks
        .iter()
        .map(|nw| {
            let is_known = known_network_names.contains(&nw.essid);
            AnnotatedWirelessNetwork::from_nw(nw.clone(), is_known)
        })
        .collect();

    if options.d() {
        dbg![&networks];
    }

    AnnotatedNetworks { networks }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::structs::WifiConnectOptions;

    #[test]
    fn test_default_network_not_known() {
        assert![!AnnotatedWirelessNetwork::default().known];
    }

    #[test]
    fn test_annotate_known() {
        let essid = "hahahahahahahahaha".to_string();
        let nw = WirelessNetwork {
            essid: essid.clone(),
            ..WirelessNetwork::default()
        };
        let known_networks = KnownNetworkNames::new(vec![nw.essid.clone()]);
        let annotated_networks = annotate_networks(
            &WifiConnectOptions::builder().build(),
            &vec![nw],
            &known_networks,
        );

        let resulting_nw = annotated_networks.networks.first().unwrap();
        assert![resulting_nw.known];
    }

    #[test]
    fn test_do_not_annotate_unknown() {
        let essid = "wheeeeeeeeeeeeeeee".to_string();
        let nw = WirelessNetwork {
            essid,
            ..WirelessNetwork::default()
        };
        let known_networks = KnownNetworkNames::default();
        let annotated_networks = annotate_networks(
            &WifiConnectOptions::builder().build(),
            &vec![nw],
            &known_networks,
        );

        let resulting_nw = annotated_networks.networks.first().unwrap();
        assert![!resulting_nw.known];
    }

    #[test]
    fn test_do_not_mangle_existing_fields() {
        let essid = "aaaaaaaaaaaaaaaaaah".to_string();
        let nw = WirelessNetwork {
            essid: essid.clone(),
            ..WirelessNetwork::default()
        };
        let known_networks = KnownNetworkNames::default();
        let annotated_networks = annotate_networks(
            &WifiConnectOptions::builder().build(),
            &vec![nw],
            &known_networks,
        );

        let resulting_nw = annotated_networks.networks.first().unwrap();
        assert_eq![essid, resulting_nw.essid];
    }

    #[test]
    fn test_do_not_mangle_essid2() {
        let essid = "guuuuuuuuuuuuuuuuuuh".to_string();
        let nw = WirelessNetwork {
            essid: essid.clone(),
            ..WirelessNetwork::default()
        };
        let known_networks = KnownNetworkNames::default();
        let annotated_networks = annotate_networks(
            &WifiConnectOptions::builder().build(),
            &vec![nw],
            &known_networks,
        );

        let resulting_nw = annotated_networks.networks.first().unwrap();
        assert![!resulting_nw.known];
    }
}
