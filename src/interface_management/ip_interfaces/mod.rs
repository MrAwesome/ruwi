mod ip_link;
use ip_link::discovery::{get_first_wifi, get_first_wired};

use crate::errors::*;
use crate::options::interfaces::Global;

pub const FAKE_INTERFACE_NAME: &str =
    "FAKE_INTERFACE_NAME_WHICH_SHOULD_ONLY_BE_SEEN_IN_TESTS_AND_DRYRUNS";

pub trait LinuxIPInterface {
    fn get_ifname(&self) -> &str;
    fn bring_up<O: Global>(&self, opts: &O) -> Result<(), RuwiError>;
    fn bring_down<O: Global>(&self, opts: &O) -> Result<(), RuwiError>;
}

#[derive(Debug, Clone)]
pub struct WifiIPInterface {
    ifname: String,
}

impl Default for WifiIPInterface {
    fn default() -> Self {
        Self {
            ifname: FAKE_INTERFACE_NAME.to_string(),
        }
    }
}

impl LinuxIPInterface  for WifiIPInterface {
    fn get_ifname(&self) -> &str {
        &self.ifname
    }

    fn bring_up<O: Global>(&self, opts: &O) -> Result<(), RuwiError> {
        ip_link::state_management::bring_up(opts, self.get_ifname())
    }

    fn bring_down<O: Global>(&self, opts: &O) -> Result<(), RuwiError> {
        ip_link::state_management::bring_down(opts, self.get_ifname())
    }
}


impl WifiIPInterface {
    pub(crate) fn new(ifname: &str) -> Self {
        Self {
            ifname: ifname.to_string(),
        }
    }

    pub(crate) fn find_first<O: Global>(opts: &O) -> Result<Self, RuwiError> {
        if opts.is_test_or_dry_run() {
            return Ok(Self::default());
        }
        let first_seen_wifi_iface = get_first_wifi(opts)?;
        Ok(Self::new(first_seen_wifi_iface.get_ifname()))
    }
}

#[derive(Debug, Clone)]
pub struct WiredIPInterface {
    ifname: String,
}

impl LinuxIPInterface  for WiredIPInterface {
    fn get_ifname(&self) -> &str {
        &self.ifname
    }
    fn bring_up<O: Global>(&self, opts: &O) -> Result<(), RuwiError> {
        ip_link::state_management::bring_up(opts, self.get_ifname())
    }

    fn bring_down<O: Global>(&self, opts: &O) -> Result<(), RuwiError> {
        ip_link::state_management::bring_down(opts, self.get_ifname())
    }
}

impl WiredIPInterface {
    pub(crate) fn new(ifname: &str) -> Self {
        Self {
            ifname: ifname.to_string(),
        }
    }

    pub(crate) fn find_first<O: Global>(opts: &O) -> Result<Self, RuwiError> {
        if opts.is_test_or_dry_run() {
            return Ok(Self::default());
        }
        let first_seen_wifi_iface = get_first_wired(opts)?;
        Ok(Self::new(first_seen_wifi_iface.get_ifname()))
    }
}

impl Default for WiredIPInterface {
    fn default() -> Self {
        Self {
            ifname: FAKE_INTERFACE_NAME.to_string(),
        }
    }
}

