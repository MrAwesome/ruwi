// TODO: rename this file to something closer to traits.rs, then find all places that break and have them use crate::prelude::*; instead
// TODO: use trait-associated functions to just have e.g. d() return self.globals().d()?
//       in other words, Find a way to reduce the need for so much new boilerplate when a new
//       method or command is added

use crate::prelude::*;

use crate::interface_management::ip_interfaces::WifiIPInterface;
use crate::known_networks::WifiKnownNetworks;

use crate::structs::ScanResult;

pub trait Global {
    fn d(&self) -> bool;
    fn get_debug(&self) -> bool;
    fn get_dry_run(&self) -> bool;
    fn get_selection_method(&self) -> &SelectionMethod;
    fn is_test_or_dry_run(&self) -> bool;
    fn pretend_to_be_root(&self) -> bool;
}

pub trait Wifi {
    fn get_scan_type(&self) -> &WifiScanType;
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
    fn get_connect_via(&self) -> &WiredConnectionType;
    fn get_given_profile_name(&self) -> &Option<String>;
}

pub trait WifiDataGatherer {
    fn get_wifi_data(
        &self,
        interface: &WifiIPInterface,
        synchronous_rescan: &Option<SynchronousRescanType>,
    ) -> Result<(WifiKnownNetworks, ScanResult), RuwiError>;
}

