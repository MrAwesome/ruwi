use crate::known_networks::WifiKnownNetworks;
use crate::prelude::*;

// TODO: make this not wifi-specific with known networks
pub(crate) fn annotate_networks<O, T, U>(
    options: &O,
    networks: &[T],
    known_networks: &WifiKnownNetworks,
) -> Vec<U>
where
    O: Global,
    T: RuwiNetwork,
    U: Annotated<T>,
{
    let networks = networks
        .iter()
        .map(|nw| {
            //let is_known = known_networks.check_for_essid(nw.get_public_name());
            // TODO: get_public_name is inefficient here - should wireless networks have their own
            // function for returning a reference to essid?
            let service_identifier =
                known_networks.get_service_identifier_for_essid(nw.get_public_name().as_ref());
            U::from_nw(nw.clone(), service_identifier)
        })
        .collect();

    if options.d() {
        dbg![&networks];
    }

    networks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::wifi::connect::WifiConnectOptions;

    #[test]
    fn test_default_network_not_known() {
        assert![!AnnotatedWirelessNetwork::from_essid_only("FAKELOL").is_known()];
    }

    #[test]
    fn test_annotate_known() {
        let essid = "hahahahahahahahaha".to_string();
        let nw = WirelessNetwork::builder().essid(essid.clone()).build();
        let known_networks = WifiKnownNetworks::new(vec![(
            essid.clone(),
            NetworkServiceIdentifier::NetworkManager,
        )]);
        let annotated_networks: Vec<AnnotatedWirelessNetwork> =
            annotate_networks(&WifiConnectOptions::default(), &[nw], &known_networks);

        let resulting_nw = annotated_networks.first().unwrap();
        assert![resulting_nw.is_known()];
        assert_eq![essid, resulting_nw.get_public_name()];
    }

    #[test]
    fn test_do_not_annotate_unknown() {
        let essid = "wheeeeeeeeeeeeeeee".to_string();
        let nw = WirelessNetwork::builder().essid(essid).build();
        let known_networks = WifiKnownNetworks::new(vec![]);
        let annotated_networks: Vec<AnnotatedWirelessNetwork> =
            annotate_networks(&WifiConnectOptions::default(), &[nw], &known_networks);

        let resulting_nw = annotated_networks.first().unwrap();
        assert![!resulting_nw.is_known()];
    }

    #[test]
    fn test_do_not_mangle_existing_fields() {
        let essid = "aaaaaaaaaaaaaaaaaah".to_string();
        let nw = WirelessNetwork::builder().essid(essid.clone()).build();
        let known_networks = WifiKnownNetworks::new(vec![]);
        let annotated_networks: Vec<AnnotatedWirelessNetwork> =
            annotate_networks(&WifiConnectOptions::default(), &[nw], &known_networks);

        let resulting_nw = annotated_networks.first().unwrap();
        assert_eq![essid, resulting_nw.get_public_name()];
    }

    #[test]
    fn test_do_not_mangle_essid2() {
        let essid = "guuuuuuuuuuuuuuuuuuh".to_string();
        let nw = WirelessNetwork::builder().essid(essid.clone()).build();
        let known_networks = WifiKnownNetworks::new(vec![]);
        let annotated_networks: Vec<AnnotatedWirelessNetwork> =
            annotate_networks(&WifiConnectOptions::default(), &[nw], &known_networks);

        let resulting_nw = annotated_networks.first().unwrap();
        assert![!resulting_nw.is_known()];
        assert_eq![essid, resulting_nw.get_public_name()];
    }
}
