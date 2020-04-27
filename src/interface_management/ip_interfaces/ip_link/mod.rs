pub(super) mod discover;
pub(super) mod state_management;
use serde_derive::Deserialize;

use crate::errors::*;
use crate::options::traits::Global;

// TODO: make sure ip is installed by default on Ubuntu, check the package name
// TODO: implement selectable
// TODO: find correct way to identify wifi vs. wired

// A direct representation of what `ip -j link show` gives back to us in JSON.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub(super) struct LinuxIPLinkInterface {
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

    pub(crate) fn is_wifi(&self) -> bool {
        let ifname = self.get_ifname();
        ifname.starts_with("wlp") || ifname.starts_with("wlan")
    }

    pub(crate) fn is_wired(&self) -> bool {
        let ifname = self.get_ifname();
        ifname.starts_with("enp") || ifname.starts_with("eth")
    }
}

pub(super) struct WifiLinuxIPLinkInterface(LinuxIPLinkInterface);
impl WifiLinuxIPLinkInterface {
    pub(super) fn get_ifname(&self) -> &str {
        self.0.get_ifname()
    }
}

impl WifiLinuxIPLinkInterface {
    pub(super) fn get_first<O: Global>(opts: &O) -> Result<Self, RuwiError> {
        let raw_interface = LinuxIPLinkInterface::get_all(opts)?
            .into_iter()
            .find(LinuxIPLinkInterface::is_wifi)
            .ok_or_else(|| rerr!(
                RuwiErrorKind::NoWifiInterfacesFound,
                "No wifi interfaces found with `ip link show`! Is \"iproute2\" installed? You can manually specify an interface with `... wifi -i <INTERFACE_NAME>`."
            ))?;
        Ok(Self(raw_interface))
    }
}

pub(super) struct WiredLinuxIPLinkInterface(LinuxIPLinkInterface);
impl WiredLinuxIPLinkInterface {
    pub(super) fn get_ifname(&self) -> &str {
        self.0.get_ifname()
    }
}

impl WiredLinuxIPLinkInterface {
    pub(super) fn get_first<O: Global>(opts: &O) -> Result<Self, RuwiError> {
        let raw_interface = LinuxIPLinkInterface::get_all(opts)?
        .into_iter()
        .find(LinuxIPLinkInterface::is_wired)
        .ok_or_else(|| rerr!(
            RuwiErrorKind::NoWiredInterfacesFound,
            "No wired interfaces found with `ip link show`! Is \"iproute2\" installed? You can manually specify an interface with `... wired -i <INTERFACE_NAME>`."
        ))?;
        Ok(Self(raw_interface))
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

        assert![dev.is_wired()];
        assert![!dev.is_wifi()];
    }

    #[test]
    fn test_wifi() {
        let dev = LinuxIPLinkInterface {
            ifname: "wlp3s0".to_string(),
            link_type: "ether".to_string(),
            operstate: OperState::UP,
            flags: vec![],
        };

        assert![dev.is_wifi()];
        assert![!dev.is_wired()];
    }
}
