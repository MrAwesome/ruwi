use crate::prelude::*;
use crate::netctl::NetctlIdentifier;

impl From<&AnnotatedWirelessNetwork> for NetctlIdentifier {
    fn from(nw: &AnnotatedWirelessNetwork) -> Self {
        let ident = match nw.get_service_identifier() {
            Some(NetworkServiceIdentifier::Netctl(ident)) => ident.clone(),
            _ => nw.get_public_name().replace(" ", "_"),
        };
        Self::new(ident)
    }
}

impl From<&AnnotatedWiredNetwork> for NetctlIdentifier {
    fn from(nw: &AnnotatedWiredNetwork) -> Self {
        let ident = match nw.get_service_identifier() {
            Some(NetworkServiceIdentifier::Netctl(ident)) => ident,
            _ => nw.get_ifname()
        };
        Self::new(ident)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface_management::ip_interfaces::WiredIPInterface;

    #[test]
    fn test_wireless_non_netctl_conversion() {
        let essid = "FU ARK";
        let nw = AnnotatedWirelessNetwork::builder()
            .essid(essid)
            .build();
        let ident = NetctlIdentifier::from(&nw);
        assert_eq!("FU_ARK", ident.as_ref());
    }
    
    #[test]
    fn test_wireless_netctl_conversion() {
        let essid = "FU ARK";
        let given_identifier = "fuark_my_home_network";
        let stored_identifier = NetworkServiceIdentifier::Netctl(given_identifier.to_string());
        let nw = AnnotatedWirelessNetwork::builder()
            .essid(essid)
            .service_identifier(stored_identifier)
            .build();
        let ident = NetctlIdentifier::from(&nw);
        assert_eq!(given_identifier, ident.as_ref());
    }

    #[test]
    fn test_wired_non_netctl_conversion() {
        let ifname = "wlp69s420";
        let interface = WiredIPInterface::new(ifname);
        let nw = AnnotatedWiredNetwork::builder()
            .interface(interface)
            .build();
        let ident = NetctlIdentifier::from(&nw);
        assert_eq!(ifname, ident.as_ref());
    }

    #[test]
    fn test_wired_netctl_conversion() {
        let ifname = "wlp69s420";
        let interface = WiredIPInterface::new(ifname);
        let given_identifier = "fuark_my_home_network";
        let stored_identifier = NetworkServiceIdentifier::Netctl(given_identifier.to_string());
        let nw = AnnotatedWiredNetwork::builder()
            .interface(interface)
            .service_identifier(stored_identifier)
            .build();
        let ident = NetctlIdentifier::from(&nw);
        assert_eq!(given_identifier, ident.as_ref());
    }
}
