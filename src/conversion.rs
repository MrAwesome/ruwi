use crate::common::*;
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
        todo!();
        let ident = match nw.get_service_identifier() {
            Some(NetworkServiceIdentifier::Netctl(ident)) => ident.clone(),
            _ => nw.get_public_name().replace(" ", "_"),
        };
        Self::new(ident)
    }
}

