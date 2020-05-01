mod ip_link;
use ip_link::{TypedLinuxInterfaceFinder, WifiLinuxIPLinkInterface, WiredLinuxIPLinkInterface};

use crate::prelude::*;

pub(crate) const DRYRUN_FAKE_INTERFACE_NAME: &str = "DRYRUN_FAKE_INTERFACE";

// TODO: better detection of whether an interface is wireless or wired
//       * find interfaces recognized by `iw`, other interfaces are likely ethernet if not loopback
//       * nmcli dev
//       * iw dev
//       * wpa_cli dev

pub(crate) trait LinuxIPInterface: Sized + From<String> {
    type Finder: TypedLinuxInterfaceFinder + Clone;

    fn get_ifname(&self) -> &str;

    fn bring_up<O: Global>(&self, opts: &O) -> Result<(), RuwiError> {
        ip_link::state_management::bring_up(opts, self.get_ifname())
    }

    fn bring_down<O: Global>(&self, opts: &O) -> Result<(), RuwiError> {
        ip_link::state_management::bring_down(opts, self.get_ifname())
    }

    fn get_all<O: Global>(opts: &O) -> Result<Vec<Self>, RuwiError> {
        let interfaces = Self::Finder::get_all(opts)?
            .into_iter()
            .map(|x| Self::from(x.get_ifname().to_string()))
            .collect();
        Ok(interfaces)
    }

    fn from_name_or_all<O: Global>(
        opts: &O,
        maybe_name: &Option<String>,
    ) -> Result<Vec<Self>, RuwiError> {
        if let Some(ifname) = maybe_name {
            Ok(vec![Self::from(ifname.clone())])
        } else {
            Self::get_all(opts)
        }
    }

    fn find_first<O: Global>(opts: &O) -> Result<Self, RuwiError> {
        if opts.is_test_or_dry_run() {
            return Ok(Self::from(DRYRUN_FAKE_INTERFACE_NAME.to_string()));
        }
        let first_seen_wifi_iface = Self::Finder::get_first(opts)?;
        Ok(Self::from(first_seen_wifi_iface.get_ifname().to_string()))
    }

    fn from_name_or_first<O: Global>(
        opts: &O,
        maybe_name: &Option<String>,
    ) -> Result<Self, RuwiError> {
        if let Some(ifname) = maybe_name {
            Ok(Self::from(ifname.clone()))
        } else {
            Self::find_first(opts)
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WifiIPInterface {
    ifname: String,
}

impl Default for WifiIPInterface {
    fn default() -> Self {
        Self::new(DRYRUN_FAKE_INTERFACE_NAME)
    }
}

impl WifiIPInterface {
    pub(crate) fn new(ifname: &str) -> Self {
        Self {
            ifname: ifname.to_string(),
        }
    }
}

impl From<String> for WifiIPInterface {
    fn from(ifname: String) -> Self {
        Self { ifname }
    }
}

impl LinuxIPInterface for WifiIPInterface {
    type Finder = WifiLinuxIPLinkInterface;

    fn get_ifname(&self) -> &str {
        &self.ifname
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WiredIPInterface {
    ifname: String,
}

impl WiredIPInterface {
    pub(crate) fn new(ifname: &str) -> Self {
        Self {
            ifname: ifname.to_string(),
        }
    }
}

impl Default for WiredIPInterface {
    fn default() -> Self {
        Self::new(DRYRUN_FAKE_INTERFACE_NAME)
    }
}

impl From<String> for WiredIPInterface {
    fn from(ifname: String) -> Self {
        Self { ifname }
    }
}

impl LinuxIPInterface for WiredIPInterface {
    type Finder = WiredLinuxIPLinkInterface;
    fn get_ifname(&self) -> &str {
        &self.ifname
    }
}
