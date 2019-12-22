use crate::interface_management::bring_interface_down;
use crate::netctl_config_writer::get_netctl_file_name;
use crate::rerr;
use crate::run_commands::*;
use crate::structs::*;

pub(crate) fn connect_to_network(
    options: &Options,
    selected_network: &AnnotatedWirelessNetwork,
    encryption_key: &Option<String>,
) -> Result<ConnectionResult, RuwiError> {
    let cv = &options.connect_via;

    match cv {
        ConnectionType::Print => {}
        conn_type @ ConnectionType::None => {
            eprintln!(
                "[NOTE]: Running in `{}` connection mode, so will not connect to: \"{}\"",
                conn_type, &selected_network.essid
            );
        }
        conn_type @ ConnectionType::Netctl | conn_type @ ConnectionType::NetworkManager => {
            eprintln!(
                "[NOTE]: Attempting to use {} to connect to: \"{}\"",
                conn_type, &selected_network.essid
            );
        }
    }

    let res = match cv {
        ConnectionType::Netctl => connect_via_netctl(options, selected_network),
        ConnectionType::NetworkManager => {
            connect_via_networkmanager(options, selected_network, encryption_key)
        }
        ConnectionType::Print => {
            let essid = selected_network.essid.clone();
            // TODO: integration tests to ensure this happens
            println!("{}", essid);
            Ok(ConnectionResult {
                connection_type: cv.clone(),
            })
        }
        ConnectionType::None => Ok(ConnectionResult {
            connection_type: ConnectionType::None,
        }),
    };

    // TODO: retry connection once if failed

    if options.debug {
        dbg![&res];
    }

    if let Ok(connection_result) = &res {
        match &connection_result.connection_type {
            conn_type @ ConnectionType::None => {
                eprintln!(
                    "[NOTE]: Running in `{}` connection mode, so did not connect to: \"{}\"",
                    conn_type, &selected_network.essid
                );
            }
            ConnectionType::Print => {}
            ConnectionType::Netctl | ConnectionType::NetworkManager => {
                eprintln!(
                    "[NOTE]: Successfully connected to: \"{}\"",
                    &selected_network.essid
                );
            }
        }
    }

    res
}

fn connect_via_netctl(
    options: &Options,
    selected_network: &AnnotatedWirelessNetwork,
) -> Result<ConnectionResult, RuwiError> {
    if options.dry_run {
        return Ok(ConnectionResult {
            connection_type: ConnectionType::Netctl,
        });
    }
    bring_interface_down(options)?;

    // TODO: don't lock so hard into filename?
    let netctl_file_name = get_netctl_file_name(&selected_network.essid);

    let res = run_command_output(options.debug, "netctl", &["switch-to", &netctl_file_name])?;

    if res.status.success() {
        Ok(ConnectionResult {
            connection_type: ConnectionType::Netctl,
        })
    } else {
        Err(rerr!(
            RuwiErrorKind::FailedToConnectViaNetctl,
            String::from_utf8_lossy(&res.stderr),
        ))
    }
}

fn connect_via_networkmanager(
    options: &Options,
    selected_network: &AnnotatedWirelessNetwork,
    encryption_key: &Option<String>,
) -> Result<ConnectionResult, RuwiError> {
    // TODO: see if interface needs to be down
    //bring_interface_down(options)?;

    // TODO TODO TODO TODO
    // TODO: for configuration, make sure NetworkManager service is running
    //
    // TODO: kill wpa_supplicant and any active netctl profiles, or print message saying to do so
    //       if connection is unsuccessful?
    // TODO TODO TODO TODO

    if options.dry_run {
        return Ok(ConnectionResult {
            connection_type: ConnectionType::NetworkManager,
        });
    }

    let args = vec!["device", "wifi", "connect", &selected_network.essid];
    let args = if let Some(pw) = encryption_key {
        let pw_args = vec!["password", pw];
        args.into_iter().chain(pw_args).collect()
    } else {
        args
    };
    let res = run_command_output(options.debug, "nmcli", &args)?;

    if res.status.success() {
        Ok(ConnectionResult {
            connection_type: ConnectionType::NetworkManager,
        })
    } else {
        Err(rerr!(
            RuwiErrorKind::FailedToConnectViaNetworkManager,
            String::from_utf8_lossy(&res.stderr),
        ))
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    //    #[test]
    //    fn test_connect_via_netctl_pass() {
    //        let opts = Options::default();
    //        let nw = AnnotatedWirelessNetwork::default();
    //        // TODO: test connect based on nw passed in
    //        let _res = connect_via_netctl(&opts, &nw);
    //        // TODO: match more robustly, compare to opts and connection type and etc
    //        //assert!(res.is_ok());
    //    }
}
