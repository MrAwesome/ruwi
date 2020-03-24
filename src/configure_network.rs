use crate::enums::*;
use crate::errors::*;
use crate::netctl_config_writer::*;
use crate::options::interfaces::*;
use crate::structs::*;
use crate::interface_management::ip_interfaces::WifiIPInterface;

pub(crate) fn possibly_configure_network<O>(
    options: &O,
    interface: &WifiIPInterface,
    network: &AnnotatedWirelessNetwork,
    encryption_key: &Option<String>,
) -> Result<Option<ConfigResult>, RuwiError>
where
    O: Global + Wifi + WifiConnect,

{
    let res = if !network.is_known() || options.get_given_encryption_key().is_some() {
        Some(configure_network(options, interface, network, encryption_key)).transpose()
    } else {
        Ok(None)
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
) -> Result<ConfigResult, RuwiError>
where
    O: Global + Wifi + WifiConnect,
{
    let cv = options.get_connect_via();
    match cv {
        WifiConnectionType::Netctl => WifiNetctlConfigHandler::new(options, interface, network, encryption_key).write(),
        WifiConnectionType::Nmcli | WifiConnectionType::None | WifiConnectionType::Print => {
            Ok(ConfigResult {
                // connection_type: cv.clone(),
                config_data: ConfigData::default(),
            })
        }
    }
}
