mod ip_link;
use ip_link::{WifiLinuxIPLinkInterface, WiredLinuxIPLinkInterface};

use crate::errors::*;
use crate::options::interfaces::Global;

pub(crate) const FAKE_INTERFACE_NAME: &str = "FAKE_INTERFACE";

pub trait LinuxIPInterface {
    fn get_ifname(&self) -> &str;
    fn from_name_or_first<O: Global>(
        opts: &O,
        maybe_name: &Option<String>,
    ) -> Result<Self, RuwiError>
    where
        Self: Sized;
    fn bring_up<O: Global>(&self, opts: &O) -> Result<(), RuwiError>;
    fn bring_down<O: Global>(&self, opts: &O) -> Result<(), RuwiError>;
}

// TODO: Remove code duplication between wifi and wired
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

impl LinuxIPInterface for WifiIPInterface {
    fn get_ifname(&self) -> &str {
        &self.ifname
    }

    fn bring_up<O: Global>(&self, opts: &O) -> Result<(), RuwiError> {
        ip_link::state_management::bring_up(opts, self.get_ifname())
    }

    fn bring_down<O: Global>(&self, opts: &O) -> Result<(), RuwiError> {
        ip_link::state_management::bring_down(opts, self.get_ifname())
    }

    fn from_name_or_first<O: Global>(
        opts: &O,
        maybe_name: &Option<String>,
    ) -> Result<Self, RuwiError> {
        if let Some(ifname) = maybe_name {
            Ok(Self::new(ifname))
        } else {
            Self::find_first(opts)
        }
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
        let first_seen_wifi_iface = WifiLinuxIPLinkInterface::get_first(opts)?;
        Ok(Self::new(first_seen_wifi_iface.get_ifname()))
    }
}

#[derive(Debug, Clone)]
pub struct WiredIPInterface {
    ifname: String,
}

impl LinuxIPInterface for WiredIPInterface {
    fn get_ifname(&self) -> &str {
        &self.ifname
    }
    fn bring_up<O: Global>(&self, opts: &O) -> Result<(), RuwiError> {
        ip_link::state_management::bring_up(opts, self.get_ifname())
    }

    fn bring_down<O: Global>(&self, opts: &O) -> Result<(), RuwiError> {
        ip_link::state_management::bring_down(opts, self.get_ifname())
    }

    fn from_name_or_first<O: Global>(
        opts: &O,
        maybe_name: &Option<String>,
    ) -> Result<Self, RuwiError> {
        if let Some(ifname) = maybe_name {
            Ok(Self::new(ifname))
        } else {
            Self::find_first(opts)
        }
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
        let first_seen_wifi_iface = WiredLinuxIPLinkInterface::get_first(opts)?;
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
