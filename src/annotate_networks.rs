use crate::known_networks::WifiKnownNetworks;
use crate::options::interfaces::*;


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
            let service_identifier = known_networks.get_service_identifier_for_essid(nw.get_public_name());
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
    use crate::structs::*;
    use crate::options::wifi::connect::WifiConnectOptions;

    #[test]
    fn test_default_network_not_known() {
        assert![!AnnotatedWirelessNetwork::default().is_known()];
    }

    #[test]
    fn test_annotate_known() {
        let essid = "hahahahahahahahaha".to_string();
        let nw = WirelessNetwork {
            essid: essid.clone(),
            ..WirelessNetwork::default()
        };
        let known_networks = WifiKnownNetworks::new(vec![(essid.clone(), essid)]);
        let annotated_networks: Vec<AnnotatedWirelessNetwork> =
            annotate_networks(&WifiConnectOptions::default(), &[nw], &known_networks);

        let resulting_nw = annotated_networks.first().unwrap();
        assert![resulting_nw.is_known()];
    }

    #[test]
    fn test_do_not_annotate_unknown() {
        let essid = "wheeeeeeeeeeeeeeee".to_string();
        let nw = WirelessNetwork {
            essid,
            ..WirelessNetwork::default()
        };
        let known_networks = WifiKnownNetworks::new(vec![]);
        let annotated_networks: Vec<AnnotatedWirelessNetwork> =
            annotate_networks(&WifiConnectOptions::default(), &[nw], &known_networks);

        let resulting_nw = annotated_networks.first().unwrap();
        assert![!resulting_nw.is_known()];
    }

    #[test]
    fn test_do_not_mangle_existing_fields() {
        let essid = "aaaaaaaaaaaaaaaaaah".to_string();
        let nw = WirelessNetwork {
            essid: essid.clone(),
            ..WirelessNetwork::default()
        };
        let known_networks = WifiKnownNetworks::new(vec![]);
        let annotated_networks: Vec<AnnotatedWirelessNetwork> =
            annotate_networks(&WifiConnectOptions::default(), &[nw], &known_networks);

        let resulting_nw = annotated_networks.first().unwrap();
        assert_eq![essid, resulting_nw.essid];
    }

    #[test]
    fn test_do_not_mangle_essid2() {
        let essid = "guuuuuuuuuuuuuuuuuuh".to_string();
        let nw = WirelessNetwork {
            essid: essid.clone(),
            ..WirelessNetwork::default()
        };
        let known_networks = WifiKnownNetworks::new(vec![]);
        let annotated_networks: Vec<AnnotatedWirelessNetwork> =
            annotate_networks(&WifiConnectOptions::default(), &[nw], &known_networks);

        let resulting_nw = annotated_networks.first().unwrap();
        assert![!resulting_nw.is_known()];
        assert_eq![essid, resulting_nw.essid];
    }
}
