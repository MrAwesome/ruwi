use crate::errors::*;
// TODO: remove reliance on naked structs here
use crate::check_known_identifiers::KnownIdentifiers;
use crate::enums::*;
use crate::structs::ScanResult;
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
}

pub trait LinuxNetworkingInterface {
    fn get_interface_name(&self) -> &str;
    fn bring_interface_up(&self) -> Result<(), RuwiError>;
    fn bring_interface_down(&self) -> Result<(), RuwiError>;
}

pub trait Wifi {
    fn get_scan_type(&self) -> &ScanType;
    fn get_scan_method(&self) -> &ScanMethod;
    fn get_ignore_known(&self) -> bool;
    fn get_force_synchronous_scan(&self) -> bool;
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

pub trait WifiDataGatherer {
    fn get_wifi_data(
        &self,
        synchronous_rescan: &Option<SynchronousRescanType>,
    ) -> Result<(KnownIdentifiers, ScanResult), RuwiError>;
}

pub trait Identifiable {
    fn get_identifier(&self) -> &str;
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
    fn from_nw(nw: T, is_known: bool) -> Self;
}

pub trait RuwiNetwork: Identifiable + Debug + Clone {}
pub trait AnnotatedRuwiNetwork: RuwiNetwork + Selectable + Known + Ord {}

pub(crate) trait GetService {
    fn get_service(&self) -> NetworkingService;
}
