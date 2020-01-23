use crate::netctl_config_writer::*;
use crate::structs::*;

pub(crate) fn possibly_configure_network(
    options: &WifiConnectOptions,
    network: &AnnotatedWirelessNetwork,
    encryption_key: &Option<String>,
) -> Result<Option<ConfigResult>, RuwiError> {
    let res = if !network.known || options.get_given_encryption_key().is_some() {
        Some(configure_network(options, network, encryption_key)).transpose()
    } else {
        Ok(None)
    };

    if options.d() {
        dbg![&res];
    }

    res
}

fn configure_network(
    options: &WifiConnectOptions,
    network: &AnnotatedWirelessNetwork,
    encryption_key: &Option<String>,
) -> Result<ConfigResult, RuwiError> {
    let cv = options.get_connect_via();
    match cv {
        WifiConnectionType::Netctl => netctl_config_write(options, network, encryption_key),
        WifiConnectionType::NetworkManager | WifiConnectionType::None | WifiConnectionType::Print => {
            Ok(ConfigResult {
                connection_type: cv.clone(),
                config_data: ConfigData::default(),
            })
        }
    }
}
