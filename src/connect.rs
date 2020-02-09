use crate::options::interfaces::*;
use crate::netctl_config_writer::get_netctl_file_name;
use crate::rerr;
use crate::run_commands::*;
use crate::errors::*;
use crate::structs::*;
use crate::service_management::GetService;

pub(crate) fn connect_to_network<O>(
    options: &O,
    selected_network: &AnnotatedWirelessNetwork,
    encryption_key: &Option<String>,
) -> Result<ConnectionResult, RuwiError> where O: Global + Wifi + WifiConnect + LinuxNetworkingInterface {
    let cv = options.get_connect_via();
    cv.get_service().start(options)?;

    match cv {
        WifiConnectionType::Print => {}
        conn_type @ WifiConnectionType::None => {
            eprintln!(
                "[NOTE]: Running in `{}` connection mode, so will not connect to: \"{}\"",
                conn_type, &selected_network.essid
            );
        }
        conn_type @ WifiConnectionType::Netctl | conn_type @ WifiConnectionType::Nmcli => {
            eprintln!(
                "[NOTE]: Attempting to use {} to connect to: \"{}\"",
                conn_type, &selected_network.essid
            );
        }
    }

    let res = match cv {
        WifiConnectionType::Netctl => connect_via_netctl(options, selected_network),
        WifiConnectionType::Nmcli => {
            connect_via_networkmanager(options, selected_network, encryption_key)
        }
        WifiConnectionType::Print => {
            let essid = selected_network.essid.clone();
            // TODO: integration tests to ensure this happens
            println!("{}", essid);
            Ok(ConnectionResult {
                connection_type: cv.clone(),
            })
        }
        WifiConnectionType::None => Ok(ConnectionResult {
            connection_type: WifiConnectionType::None,
        }),
    };

    // TODO: retry connection once if failed

    if options.d() {
        dbg![&res];
    }

    if let Ok(connection_result) = &res {
        match &connection_result.connection_type {
            conn_type @ WifiConnectionType::None => {
                eprintln!(
                    "[NOTE]: Running in `{}` connection mode, so did not connect to: \"{}\"",
                    conn_type, &selected_network.essid
                );
            }
            WifiConnectionType::Print => {}
            WifiConnectionType::Netctl | WifiConnectionType::Nmcli => {
                eprintln!(
                    "[NOTE]: Successfully connected to: \"{}\"",
                    &selected_network.essid
                );
            }
        }
    }

    res
}

fn connect_via_netctl<O>(
    options: &O,
    selected_network: &AnnotatedWirelessNetwork,
) -> Result<ConnectionResult, RuwiError> where O: Global + LinuxNetworkingInterface {
    if options.get_dry_run() {
        return Ok(ConnectionResult {
            connection_type: WifiConnectionType::Netctl,
        });
    }
    options.bring_interface_down()?;

    // TODO: don't lock so hard into filename?
    let netctl_file_name = get_netctl_file_name(&selected_network.essid);

    let res = run_command_output(options, "netctl", &["switch-to", &netctl_file_name])?;

    if res.status.success() {
        Ok(ConnectionResult {
            connection_type: WifiConnectionType::Netctl,
        })
    } else {
        Err(rerr!(
            RuwiErrorKind::FailedToConnectViaNetctl,
            String::from_utf8_lossy(&res.stderr),
        ))
    }
}

fn connect_via_networkmanager<O>(
    options: &O,
    selected_network: &AnnotatedWirelessNetwork,
    encryption_key: &Option<String>,
) -> Result<ConnectionResult, RuwiError> where O: Global {
    // TODO: see if interface needs to be down
    //bring_interface_down(options)?;

    // TODO TODO TODO TODO
    // TODO: for configuration, make sure NetworkManager service is running
    //
    // TODO: kill wpa_supplicant and any active netctl profiles, or print message saying to do so
    //       if connection is unsuccessful?
    // TODO TODO TODO TODO

    if options.get_dry_run() {
        return Ok(ConnectionResult {
            connection_type: WifiConnectionType::Nmcli,
        });
    }

    let args = vec!["device", "wifi", "connect", &selected_network.essid];
    let args = if let Some(pw) = encryption_key {
        let pw_args = vec!["password", pw];
        args.into_iter().chain(pw_args).collect()
    } else {
        args
    };
    let res = run_command_output(options, "nmcli", &args)?;

    if res.status.success() {
        Ok(ConnectionResult {
            connection_type: WifiConnectionType::Nmcli,
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
    //        let opts = WifiConnectOptions::default();
    //        let nw = AnnotatedWirelessNetwork::default();
    //        // TODO: test connect based on nw passed in
    //        let _res = connect_via_netctl(&opts, &nw);
    //        // TODO: match more robustly, compare to opts and connection type and etc
    //        //assert!(res.is_ok());
    //    }
}
