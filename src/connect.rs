use crate::interface_management::bring_interface_down;
use crate::netctl_config_writer::get_netctl_file_name;
use crate::run_commands::*;
use crate::structs::*;
use std::error::Error;

pub(crate) fn connect_to_network(
    options: &Options,
    selected_network: &AnnotatedWirelessNetwork,
) -> Result<ConnectionResult, Box<dyn Error + Send + Sync>> {
    let res = match &options.connect_via {
        ConnectionType::Netctl => connect_via_netctl(options, selected_network),
        ConnectionType::None => Ok(ConnectionResult {
            connection_type: ConnectionType::None,
            cmd_output: None,
        }),
        x @ _ => Err(nie(x)),
    };

    if options.debug {
        dbg![&res];
    }

    res
}

fn connect_via_netctl(
    options: &Options,
    selected_network: &AnnotatedWirelessNetwork,
) -> Result<ConnectionResult, Box<dyn Error + Send + Sync>> {
    bring_interface_down(options)?;

    let netctl_file_name = get_netctl_file_name(&selected_network.essid);

    let netctl_connect_output = 
        run_command_pass_stdout(options.debug,
            "netctl",
            &["switch-to", &netctl_file_name],
            &"Failed to connect. Check `journalctl -xe` for details, or try running again with -d to see more information.",
        )?;

    Ok(ConnectionResult {
        connection_type: ConnectionType::Netctl,
        cmd_output: Some(netctl_connect_output),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect_via_netctl_pass() {
        let opts = Options::default();
        let nw = AnnotatedWirelessNetwork::default();
        // TODO: test connect based on nw passed in
        let res = connect_via_netctl(&opts, &nw);
        // TODO: match more robustly, compare to opts and connection type and etc
        //assert!(res.is_ok());
    }
}
