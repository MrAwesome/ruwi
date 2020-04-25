use crate::enums::*;
use crate::errors::*;
use crate::interface_management::ip_interfaces::WifiIPInterface;
use crate::netctl::*;
use crate::options::interfaces::*;
use crate::structs::*;

pub(crate) fn possibly_configure_network<O>(
    options: &O,
    interface: &WifiIPInterface,
    network: &AnnotatedWirelessNetwork,
    encryption_key: &Option<String>,
) -> Result<(), RuwiError>
where
    O: Global + Wifi + WifiConnect,
{
    let res = if !network.is_known() || options.get_given_encryption_key().is_some() {
        configure_network(options, interface, network, encryption_key)
    } else {
        Ok(())
    };

    if options.d() {
        dbg![&res];
    }

    res
}

fn configure_network<O>(
    options: &O,
    interface: &WifiIPInterface,
    network: &AnnotatedWirelessNetwork,
    encryption_key: &Option<String>,
) -> Result<(), RuwiError>
where
    O: Global + Wifi + WifiConnect,
{
    let cv = options.get_connect_via();
    match cv {
        WifiConnectionType::Netctl => NetctlConfigHandler::new(options)
            .write_wifi_config(interface, network, encryption_key)
            .and(Ok(())),
        WifiConnectionType::Nmcli | WifiConnectionType::None | WifiConnectionType::Print => Ok(()),
    }
}
