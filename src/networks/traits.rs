use crate::enums::*;
use crate::interface_management::ip_interfaces::*;
use crate::netctl::NetctlIdentifier;

use std::fmt::Debug;

pub trait Identifiable {
    // For wifi, this is ESSID.
    fn get_public_name(&self) -> &str;
}

pub trait Known {
    fn is_known(&self) -> bool;
    fn get_service_identifier(&self) -> Option<&NetworkServiceIdentifier>;
}

pub trait Selectable {
    fn get_display_string(&self) -> String;
}

// This exists so that AnnotatedRuwiNetwork does not need to have the
// associated type defined everywhere it is used, since associated trait
// bounds are unstable right now (Q1 2020).
pub trait Annotated<T>: Known + Debug {
    fn from_nw(nw: T, service_identifier: Option<&NetworkServiceIdentifier>) -> Self;
}

pub trait RuwiNetwork: Identifiable + Debug + Clone {}
pub trait AnnotatedRuwiNetwork: RuwiNetwork + Selectable + Known + Ord {}

// TODO: Should this exist?
pub(crate) trait GetService {
    fn get_service(&self, interface: Option<&WifiIPInterface>) -> NetworkingService;
}

pub(crate) trait HasInterface: RuwiNetwork {
    fn get_interface<T: LinuxIPInterface>(&self) -> &T;
}

pub(crate) trait ToNetctlIdentifier: AnnotatedRuwiNetwork + HasInterface {
    fn get_netctl_identifier(&self) -> NetctlIdentifier;
}
