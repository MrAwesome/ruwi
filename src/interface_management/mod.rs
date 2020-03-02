pub(crate) mod interface_discovery;
pub(crate) mod linux_networking_interface_management;

use crate::errors::*;
use crate::options::interfaces::Global;

use interface_discovery::network_interfaces::*;
use linux_networking_interface_management::*;

use serde_derive::Deserialize;

#[derive(Debug, Clone)]
pub struct WifiIPInterface {
    ifname: String,
}

impl Default for WifiIPInterface {
    fn default() -> Self {
        Self {
            ifname: "wlan0".to_string(),
        }
    }
}

impl WifiIPInterface {
    pub(crate) fn new(ifname: &str) -> Self {
        Self {
            ifname: ifname.to_string(),
        }
    }

    pub(crate) fn get_ifname(&self) -> &str {
        &self.ifname
    }

    pub(crate) fn find_first<O: Global>(opts: &O) -> Result<Self, RuwiError> {
        if opts.is_test_or_dry_run() {
            return Ok(Self::default());
        }
        let first_seen_wifi_iface = LinuxIPLinkInterface::get_first_wifi(opts)?;
        Ok(Self::new(first_seen_wifi_iface.get_ifname()))
    }

    pub(crate) fn bring_up<O: Global>(&self, opts: &O) -> Result<(), RuwiError> {
        if opts.is_test_or_dry_run() {
            return Ok(());
        }
        let interface = LinuxIPLinkInterface::get_by_name(opts, self.get_ifname())?;
        interface.set_up(opts)?;
        Ok(())
    }

    pub(crate) fn bring_down<O: Global>(&self, opts: &O) -> Result<(), RuwiError> {
        if opts.is_test_or_dry_run() {
            return Ok(());
        }
        let interface = LinuxIPLinkInterface::get_by_name(opts, self.get_ifname())?;
        interface.set_down(opts)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct WiredIPInterface {
    ifname: String,
}

impl WiredIPInterface {
    pub(crate) fn new(ifname: &str) -> Self {
        Self {
            ifname: ifname.to_string(),
        }
    }

    pub(crate) fn get_ifname(&self) -> &str {
        &self.ifname
    }

    pub(crate) fn find_first<O: Global>(opts: &O) -> Result<Self, RuwiError> {
        if opts.is_test_or_dry_run() {
            return Ok(Self::default());
        }
        let first_seen_wifi_iface = LinuxIPLinkInterface::get_first_wired(opts)?;
        Ok(Self::new(first_seen_wifi_iface.get_ifname()))
    }

    pub(crate) fn bring_up<O: Global>(&self, opts: &O) -> Result<(), RuwiError> {
        if ! opts.get_dry_run() {
            return Ok(());
        }
        let interface = LinuxIPLinkInterface::get_by_name(opts, self.get_ifname())?;
        interface.set_up(opts)?;
        Ok(())
    }

    pub(crate) fn bring_down<O: Global>(&self, opts: &O) -> Result<(), RuwiError> {
        if opts.get_dry_run() {
            return Ok(());
        }
        let interface = LinuxIPLinkInterface::get_by_name(opts, self.get_ifname())?;
        interface.set_down(opts)?;
        Ok(())
    }
}

impl Default for WiredIPInterface {
    fn default() -> Self {
        Self {
            ifname: "eth0".to_string(),
        }
    }
}

// TODO: do create separate types for Wifi and Wired, and have the defaults for each match the correct names
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
struct LinuxIPLinkInterface {
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
    pub(crate) fn get_by_name<O: Global>(opts: &O, name: &str) -> Result<Self, RuwiError> {
        get_interface_by_name(opts, name)
    }

    pub(crate) fn get_first_wifi<O: Global>(opts: &O) -> Result<Self, RuwiError> {
        get_all_interfaces(opts)?.into_iter().find(Self::is_wifi).ok_or_else(|| rerr!(RuwiErrorKind::NoWifiInterfacesFound, "No wifi interfaces found with `ip link show`! Is \"iproute2\" installed? You can manually specify an interface with `... wifi -i <INTERFACE_NAME>`."))
    }

    pub(crate) fn get_first_wired<O: Global>(opts: &O) -> Result<Self, RuwiError> {
        get_all_interfaces(opts)?.into_iter().find(Self::is_wired).ok_or_else(|| rerr!(RuwiErrorKind::NoWiredInterfacesFound, "No wired interfaces found with `ip link show`! Is \"iproute2\" installed? You can manually specify an interface with `... wired -i <INTERFACE_NAME>`."))
    }

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
