use crate::netctl_config_writer::get_netctl_file_name;
use crate::structs::*;
use std::io;
use std::process::{Command, Stdio};

// TODO: start netctl/networkmanager if they aren't already running and connection is requested

pub fn connect_to_network(
    options: &Options,
    selected_network: &WirelessNetwork,
) -> io::Result<ConnectionResult> {
    // TODO: implement
    let res = match &options.connect_via {
        ConnectionType::Netctl => connect_via_netctl(options, selected_network),
        ConnectionType::None => Ok(ConnectionResult {
            connection_type: ConnectionType::None,
            cmd_output: None,
        }),
        // TODO: implement
        x @ _ => Err(nie(x)),
    };
    if options.debug {
        dbg!(&res);
    }

    res
}

pub fn connect_via_netctl(
    _options: &Options,
    selected_network: &WirelessNetwork,
) -> io::Result<ConnectionResult> {
    let netctl_file_name = get_netctl_file_name(&selected_network.essid);
    let output = Command::new("netctl")
        .arg("switch-to")
        .arg(&netctl_file_name)
        .stdout(Stdio::piped())
        .output()?;
    // TODO: check for exit status and return scanerror if nonzero
    Ok(ConnectionResult {
        connection_type: ConnectionType::Netctl,
        cmd_output: Some(output),
    })
}
