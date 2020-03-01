pub(crate) mod get_default_interface;
pub(crate) mod interface_discovery;
pub(crate) mod linux_networking_interface_management;

use crate::errors::*;
use crate::options::interfaces::Global;

use interface_discovery::network_interfaces::*;
use linux_networking_interface_management::*;

use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct LinuxIPLinkDevice {
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

impl LinuxIPLinkDevice {
    pub(crate) fn is_up(&self) -> bool {
        self.operstate == OperState::UP || self.flags.iter().any(|x| x == "UP")
    }
    pub(crate) fn is_down(&self) -> bool {
        !self.is_up()
    }
    pub(crate) fn set_up<O: Global>(self, opts: &O) -> Result<Self, RuwiError> {
        let ifname = self.get_ifname();
        bring_linux_networking_interface_up(opts, ifname)?;
        get_interface_by_name(opts, ifname)
    }
    pub(crate) fn set_down<O: Global>(self, opts: &O) -> Result<Self, RuwiError> {
        let ifname = self.get_ifname();
        bring_linux_networking_interface_down(opts, ifname)?;
        get_interface_by_name(opts, ifname)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wired() {
        let dev = LinuxIPLinkDevice {
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
        let dev = LinuxIPLinkDevice {
            ifname: "wlp3s0".to_string(),
            link_type: "ether".to_string(),
            operstate: OperState::UP,
            flags: vec![],
        };

        assert![dev.is_wifi()];
        assert![!dev.is_wired()];
    }
}
