use crate::netctl_config_writer::*;
use crate::structs::*;

// For multiple outputs:
//pub(crate) fn send_outputs(
//    options: &Options,
//    network: &AnnotatedWirelessNetwork,
//    encryption_key: &Option<String>,
//) -> Vec<Result<OutputResult, RuwiError>> {
//    options
//        .output_types
//        .iter()
//        .map(|opt| send_output(options, opt, network, encryption_key))
//        .collect()
//}

// TODO: Still do output for types which aren't config-based (like "print selected network")?
//       Or should they be a separate command?
pub(crate) fn possibly_configure_network(
    options: &Options,
    network: &AnnotatedWirelessNetwork,
    encryption_key: &Option<String>,
) -> Result<Option<ConfigResult>, RuwiError> {
    let res = if !network.known || options.given_encryption_key.is_some() {
        Some(configure_network(options, network, encryption_key)).transpose()
    } else {
        Ok(None)
    };

    if options.debug {
        dbg![&res];
    }

    res
}

fn configure_network(
    options: &Options,
    network: &AnnotatedWirelessNetwork,
    encryption_key: &Option<String>,
) -> Result<ConfigResult, RuwiError> {
    match &options.connect_via {
        ConnectionType::Netctl => netctl_config_write(options, network, encryption_key),
        x @ ConnectionType::NetworkManager | x @ ConnectionType::None => Ok(ConfigResult {
            connection_type: x.clone(),
            config_data: Default::default(),
        }),
    }
}
