// TODO: rename this file to something closer to context_traits
// TODO: use trait-associated functions to just have e.g. d() return self.globals().d()?
//       in other words, Find a way to reduce the need for so much new boilerplate when a new
//       method or command is added

use crate::errors::*;
// TODO: remove reliance on naked structs here
use crate::known_networks::WifiKnownNetworks;
use crate::enums::*;
use crate::structs::ScanResult;
use crate::interface_management::ip_interfaces::WifiIPInterface;
use std::fmt::Debug;

// TODO: Remove networks from here and put elsewhere

// TODO: make more usages look like this
// impl<O> NetworkingService where O: Global {

pub trait Global {
    fn d(&self) -> bool;
    fn get_debug(&self) -> bool;
    fn get_dry_run(&self) -> bool;
    fn get_selection_method(&self) -> &SelectionMethod;
    fn is_test_or_dry_run(&self) -> bool;
    fn pretend_to_be_root(&self) -> bool;
}

pub trait Wifi {
    fn get_scan_type(&self) -> &ScanType;
    fn get_scan_method(&self) -> &ScanMethod;
    fn get_ignore_known(&self) -> bool;
    fn get_force_synchronous_scan(&self) -> bool;
    fn get_given_interface_name(&self) -> &Option<String>;
}

pub trait Wired {
    fn get_given_interface_name(&self) -> &Option<String>;
}

pub trait AutoSelect {
    fn get_auto_mode(&self) -> &AutoMode;
}

pub trait WifiConnect {
    fn get_force_ask_password(&self) -> bool;
    fn get_given_essid(&self) -> &Option<String>;
    fn get_given_encryption_key(&self) -> &Option<String>;
    fn get_connect_via(&self) -> &WifiConnectionType;
}

pub trait WiredConnect {
    fn get_connect_via(&self) -> &RawInterfaceConnectionType;
}

pub trait WifiDataGatherer {
    fn get_wifi_data(
        &self,
        interface: &WifiIPInterface,
        synchronous_rescan: &Option<SynchronousRescanType>,
    ) -> Result<(WifiKnownNetworks, ScanResult), RuwiError>;
}

pub trait Identifiable {
    // For wifi, this is ESSID.
    fn get_public_name(&self) -> &str;
}

pub trait Known {
    fn is_known(&self) -> bool;
}

pub trait Selectable {
    fn get_display_string(&self) -> String;
}

// This exists so that AnnotatedRuwiNetwork does not need to have the
// associated type defined everywhere it is used, since associated trait
// bounds are unstable right now (Q1 2020).
pub trait Annotated<T>: Known + Debug {
    fn from_nw(nw: T, service_identifier: Option<&str>) -> Self;
}

pub trait RuwiNetwork: Identifiable + Debug + Clone {}
pub trait AnnotatedRuwiNetwork: RuwiNetwork + Selectable + Known + Ord {}

pub(crate) trait GetService {
    fn get_service(&self) -> NetworkingService;
}
