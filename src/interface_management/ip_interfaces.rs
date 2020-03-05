use crate::errors::*;
use crate::options::interfaces::Global;

use super::interface_discovery::network_interfaces::*;
use super::linux_networking_interface_management::*;

use serde_derive::Deserialize;

const FAKE_INTERFACE_NAME: &str =
    "FAKE_INTERFACE_NAME_WHICH_SHOULD_ONLY_BE_SEEN_IN_TESTS_AND_DRYRUNS";

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

impl Default for WiredIPInterface {
    fn default() -> Self {
        Self {
            ifname: FAKE_INTERFACE_NAME.to_string(),
        }
    }
}

// A direct representation of what `ip -j link show` gives back to us in JSON.
//
// WiredLinuxIPLinkInterface and WifiLinuxIPLinkInterface are simple type wrappers
// to give us type safety in the conversions into more publicly-visible interface types
// (your dear author was bitten by the stringly-typed get_first_wi{fi,red} functions
// being so visually similar and returning the same type).
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub(super) struct LinuxIPLinkInterface {
    pub(super) ifname: String,
    pub(super) link_type: String,
    pub(super) operstate: OperState,
    pub(super) flags: Vec<String>,
}

pub(super) struct WifiLinuxIPLinkInterface(LinuxIPLinkInterface);
impl WifiLinuxIPLinkInterface {
    fn get_ifname(&self) -> &str {
        self.0.get_ifname()
    }
}

pub(super) struct WiredLinuxIPLinkInterface(LinuxIPLinkInterface);
impl WiredLinuxIPLinkInterface {
    fn get_ifname(&self) -> &str {
        self.0.get_ifname()
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(field_identifier)]
pub(super) enum OperState {
    UP,
    DOWN,
    UNKNOWN,
    Other(String),
}

impl LinuxIPLinkInterface {
    pub(crate) fn get_by_name<O: Global>(opts: &O, name: &str) -> Result<Self, RuwiError> {
        get_interface_by_name(opts, name)
    }

    pub(super) fn get_first_wifi<O: Global>(
        opts: &O,
    ) -> Result<WifiLinuxIPLinkInterface, RuwiError> {
        let raw_interface = get_all_interfaces(opts)?
            .into_iter()
            .find(Self::is_wifi)
            .ok_or_else(|| rerr!(
                RuwiErrorKind::NoWifiInterfacesFound,
                "No wifi interfaces found with `ip link show`! Is \"iproute2\" installed? You can manually specify an interface with `... wifi -i <INTERFACE_NAME>`."
            ))?;
        Ok(WifiLinuxIPLinkInterface(raw_interface))
    }

    #[cfg(test)]
    fn TODO_TEST_FIRST_WIRED_ETC() {}

    pub(super) fn get_first_wired<O: Global>(
        opts: &O,
    ) -> Result<WiredLinuxIPLinkInterface, RuwiError> {
        let raw_interface = get_all_interfaces(opts)?
            .into_iter()
            .find(Self::is_wired)
            .ok_or_else(|| rerr!(
                RuwiErrorKind::NoWiredInterfacesFound,
                "No wired interfaces found with `ip link show`! Is \"iproute2\" installed? You can manually specify an interface with `... wired -i <INTERFACE_NAME>`."
            ))?;
        Ok(WiredLinuxIPLinkInterface(raw_interface))
    }

    fn _is_up(&self) -> bool {
        self.operstate == OperState::UP || self.flags.iter().any(|x| x == "UP")
    }
    fn _is_down(&self) -> bool {
        !self._is_up()
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
