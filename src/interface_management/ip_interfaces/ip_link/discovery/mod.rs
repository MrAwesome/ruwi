mod discover;
use discover::*;

use super::*;
use crate::options::interfaces::Global;
use crate::errors::*;

pub(in super::super) fn get_wifi_by_name<O: Global>(opts: &O, name: &str) -> Result<WifiLinuxIPLinkInterface, RuwiError> {
    Ok(WifiLinuxIPLinkInterface(get_interface_by_name(opts, name)?))
}

pub(in super::super) fn get_wired_by_name<O: Global>(opts: &O, name: &str) -> Result<WiredLinuxIPLinkInterface, RuwiError> {
    Ok(WiredLinuxIPLinkInterface(get_interface_by_name(opts, name)?))
}

pub(in super::super) fn get_first_wifi<O: Global>(
    opts: &O,
) -> Result<WifiLinuxIPLinkInterface, RuwiError> {
    let raw_interface = get_all_interfaces(opts)?
        .into_iter()
        .find(LinuxIPLinkInterface::is_wifi)
        .ok_or_else(|| rerr!(
            RuwiErrorKind::NoWifiInterfacesFound,
            "No wifi interfaces found with `ip link show`! Is \"iproute2\" installed? You can manually specify an interface with `... wifi -i <INTERFACE_NAME>`."
        ))?;
    Ok(WifiLinuxIPLinkInterface(raw_interface))
}

#[cfg(test)]
fn TODO_TEST_FIRST_WIRED_ETC() {}

pub(in super::super) fn get_first_wired<O: Global>(
    opts: &O,
) -> Result<WiredLinuxIPLinkInterface, RuwiError> {
    let raw_interface = get_all_interfaces(opts)?
        .into_iter()
        .find(LinuxIPLinkInterface::is_wired)
        .ok_or_else(|| rerr!(
            RuwiErrorKind::NoWiredInterfacesFound,
            "No wired interfaces found with `ip link show`! Is \"iproute2\" installed? You can manually specify an interface with `... wired -i <INTERFACE_NAME>`."
        ))?;
    Ok(WiredLinuxIPLinkInterface(raw_interface))
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
