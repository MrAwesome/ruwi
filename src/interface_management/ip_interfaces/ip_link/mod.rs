pub(super) mod discover;
pub(super) mod state_management;

use crate::prelude::*;

use serde_derive::Deserialize;
use std::convert::TryFrom;

pub(crate) struct InterfaceNotDesiredTypeError;

// NOTE: This covers both the classic "wlan" and the newer "wl*"
//       This does not cover WWAN networks, as the author doesn't have
//       any hardware available to test those connections. Feel free to
//       try connecting to wwan directly with ruwi by specifying the "ww*"
//       interface with -i, and file an issue (or a PR!) either way.
fn is_wireless(ifname: &str) -> bool {
    ifname.starts_with("wl")
}

fn is_ethernet(ifname: &str) -> bool {
    ifname.starts_with("en") || ifname.starts_with("eth")
}

// This trait is used to find all Linux networking interfaces of a
// certain type (Wireless, Ethernet, etc).
//
// For more info on the naming scheme for networking interfaces,
// see `man systemd.net-naming-scheme`.
pub(crate) trait TypedLinuxInterfaceFinder: TryFrom<LinuxIPLinkInterface> {
    fn get_ifname(&self) -> &str;
    fn check(untyped_interface: &LinuxIPLinkInterface) -> bool;
    fn none_found_error() -> RuwiError;

    fn get_first<O: Global>(opts: &O) -> Result<Self, RuwiError>
    where
        Self: Sized + Clone,
    {
        Self::get_all(opts)?
            .first()
            .ok_or_else(Self::none_found_error)
            .map(Clone::clone)
    }

    fn get_all<O: Global>(opts: &O) -> Result<Vec<Self>, RuwiError>
    where
        Self: Sized,
    {
        let raw_interfaces = LinuxIPLinkInterface::get_all(opts)?
            .into_iter()
            .filter_map(|x| Self::try_from(x).ok())
            .collect::<Vec<_>>();
        Ok(raw_interfaces)
    }
}

// TODO: make sure ip is installed by default on Ubuntu, check the package name
// TODO: implement selectable
// TODO: find correct way to identify wifi vs. wired

// A direct representation of what `ip -j link show` gives back to us in JSON.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub(crate) struct LinuxIPLinkInterface {
    ifname: String,
    link_type: String,
    operstate: OperState,
    flags: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(field_identifier)]
enum OperState {
    UP,
    DOWN,
    UNKNOWN,
    Other(String),
}

impl LinuxIPLinkInterface {
    fn _is_up(&self) -> bool {
        self.operstate == OperState::UP || self.flags.iter().any(|x| x == "UP")
    }
    fn _is_down(&self) -> bool {
        !self._is_up()
    }

    pub(crate) fn get_ifname(&self) -> &str {
        &self.ifname
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WifiLinuxIPLinkInterface(LinuxIPLinkInterface);

impl TryFrom<LinuxIPLinkInterface> for WifiLinuxIPLinkInterface {
    type Error = InterfaceNotDesiredTypeError;
    fn try_from(
        untyped_interface: LinuxIPLinkInterface,
    ) -> Result<Self, InterfaceNotDesiredTypeError> {
        if Self::check(&untyped_interface) {
            Ok(Self(untyped_interface))
        } else {
            Err(InterfaceNotDesiredTypeError)
        }
    }
}

// TODO: better detection of whether an interface is wireless or wired
//       * find interfaces recognized by `iw`, other interfaces are likely ethernet if not loopback
//       * nmcli dev
//       * iw dev
//       * wpa_cli dev

impl TypedLinuxInterfaceFinder for WifiLinuxIPLinkInterface {
    fn get_ifname(&self) -> &str {
        self.0.get_ifname()
    }

    fn check(untyped_interface: &LinuxIPLinkInterface) -> bool {
        let ifname = untyped_interface.get_ifname();
        is_wireless(ifname)
    }

    fn none_found_error() -> RuwiError {
        rerr!(
            RuwiErrorKind::NoWifiInterfacesFound,
            "No wifi interfaces found with `ip link show`! Is \"iproute2\" installed? You can manually specify an interface with `... wifi -i <INTERFACE_NAME>`."
        )
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WiredLinuxIPLinkInterface(LinuxIPLinkInterface);

impl TryFrom<LinuxIPLinkInterface> for WiredLinuxIPLinkInterface {
    type Error = InterfaceNotDesiredTypeError;
    fn try_from(
        untyped_interface: LinuxIPLinkInterface,
    ) -> Result<Self, InterfaceNotDesiredTypeError> {
        if Self::check(&untyped_interface) {
            Ok(Self(untyped_interface))
        } else {
            Err(InterfaceNotDesiredTypeError)
        }
    }
}

impl TypedLinuxInterfaceFinder for WiredLinuxIPLinkInterface {
    fn get_ifname(&self) -> &str {
        self.0.get_ifname()
    }

    fn check(untyped_interface: &LinuxIPLinkInterface) -> bool {
        let ifname = untyped_interface.get_ifname();
        is_ethernet(ifname)
    }

    fn none_found_error() -> RuwiError {
        rerr!(
            RuwiErrorKind::NoWiredInterfacesFound,
            "No wired interfaces found with `ip link show`! Is \"iproute2\" installed? You can manually specify an interface with `... wired -i <INTERFACE_NAME>`."
        )
    }
}

// TODO: test wired::get_first and wifi::get_first
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wired() {
        let dev = LinuxIPLinkInterface {
            ifname: "enp0s25".to_string(),
            link_type: "ether".to_string(),
            operstate: OperState::UP,
            flags: vec![],
        };

        assert![WiredLinuxIPLinkInterface::try_from(dev.clone()).is_ok()];
        assert![!WifiLinuxIPLinkInterface::try_from(dev).is_ok()];
    }

    #[test]
    fn test_wired_classic() {
        let dev = LinuxIPLinkInterface {
            ifname: "eth0".to_string(),
            link_type: "ether".to_string(),
            operstate: OperState::UP,
            flags: vec![],
        };

        assert![WiredLinuxIPLinkInterface::try_from(dev.clone()).is_ok()];
        assert![!WifiLinuxIPLinkInterface::try_from(dev).is_ok()];
    }

    #[test]
    fn test_wifi() {
        let dev = LinuxIPLinkInterface {
            ifname: "wlp3s0".to_string(),
            link_type: "ether".to_string(),
            operstate: OperState::UP,
            flags: vec![],
        };

        assert![WifiLinuxIPLinkInterface::try_from(dev.clone()).is_ok()];
        assert![!WiredLinuxIPLinkInterface::try_from(dev).is_ok()];
    }

    #[test]
    fn test_wifi_classic() {
        let dev = LinuxIPLinkInterface {
            ifname: "wlan0".to_string(),
            link_type: "ether".to_string(),
            operstate: OperState::UP,
            flags: vec![],
        };

        assert![WifiLinuxIPLinkInterface::try_from(dev.clone()).is_ok()];
        assert![!WiredLinuxIPLinkInterface::try_from(dev).is_ok()];
    }
}
