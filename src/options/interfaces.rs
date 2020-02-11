use crate::errors::*;
use crate::structs::*;

// TODO: make more usages look like this
// impl<O> NetworkingService where O: Global {

pub trait Global {
    fn d(&self) -> bool;
    fn get_debug(&self) -> bool;
    fn get_dry_run(&self) -> bool;
    fn get_selection_method(&self) -> &SelectionMethod;
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

pub trait Identifiable {
    fn get_identifier(&self) -> &str;
}

pub trait Annotated<T> {
    fn from(orignal_type: T, is_known: bool) -> Self;
    fn is_known(&self) -> bool;
}
