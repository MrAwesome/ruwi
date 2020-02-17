use crate::check_known_identifiers::KnownIdentifiers;
use crate::options::interfaces::*;

pub(crate) fn annotate_networks<O, T, U>(
    options: &O,
    networks: &[T],
    known_identifiers: &KnownIdentifiers,
) -> Vec<U>
where
    O: Global,
    T: RuwiNetwork,
    U: Annotated<T>,
{
    let networks = networks
        .iter()
        .map(|nw| {
            let is_known = known_identifiers.check_for(nw.get_identifier());
            U::from_nw(nw.clone(), is_known)
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
        assert![!AnnotatedWirelessNetwork::default().known];
    }

    #[test]
    fn test_annotate_known() {
        let essid = "hahahahahahahahaha".to_string();
        let nw = WirelessNetwork {
            essid: essid.clone(),
            ..WirelessNetwork::default()
        };
        let known_networks = KnownIdentifiers::new(vec![essid]);
        let annotated_networks: Vec<AnnotatedWirelessNetwork> =
            annotate_networks(&WifiConnectOptions::default(), &[nw], &known_networks);

        let resulting_nw = annotated_networks.first().unwrap();
        assert![resulting_nw.known];
    }

    #[test]
    fn test_do_not_annotate_unknown() {
        let essid = "wheeeeeeeeeeeeeeee".to_string();
        let nw = WirelessNetwork {
            essid,
            ..WirelessNetwork::default()
        };
        let known_networks = KnownIdentifiers::new(vec![]);
        let annotated_networks: Vec<AnnotatedWirelessNetwork> =
            annotate_networks(&WifiConnectOptions::default(), &[nw], &known_networks);

        let resulting_nw = annotated_networks.first().unwrap();
        assert![!resulting_nw.known];
    }

    #[test]
    fn test_do_not_mangle_existing_fields() {
        let essid = "aaaaaaaaaaaaaaaaaah".to_string();
        let nw = WirelessNetwork {
            essid: essid.clone(),
            ..WirelessNetwork::default()
        };
        let known_networks = KnownIdentifiers::new(vec![]);
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
        let known_networks = KnownIdentifiers::new(vec![]);
        let annotated_networks: Vec<AnnotatedWirelessNetwork> =
            annotate_networks(&WifiConnectOptions::default(), &[nw], &known_networks);

        let resulting_nw = annotated_networks.first().unwrap();
        assert![!resulting_nw.known];
        assert_eq![essid, resulting_nw.essid];
    }
}
