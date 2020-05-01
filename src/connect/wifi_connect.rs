use crate::prelude::*;

use crate::interface_management::ip_interfaces::{LinuxIPInterface, WifiIPInterface};
use crate::run_commands::SystemCommandRunner;
use crate::netctl::utils::netctl_switch_to;
use crate::netctl::NetctlIdentifier;

pub(crate) fn connect_to_network<O>(
    options: &O,
    interface: &WifiIPInterface,
    selected_network: &AnnotatedWirelessNetwork,
    encryption_key: &Option<String>,
) -> Result<ConnectionResult, RuwiError>
where
    O: Global + Wifi + WifiConnect,
{
    manage_services(options, interface)?;

    let connect_via = options.get_connect_via();
    match connect_via {
        WifiConnectionType::Print => {}
        conn_type @ WifiConnectionType::None => {
            eprintln!(
                "[NOTE]: Running in `{}` connection mode, so will not connect to: \"{}\"",
                conn_type,
                selected_network.get_public_name()
            );
        }
        conn_type @ WifiConnectionType::Netctl | conn_type @ WifiConnectionType::Nmcli => {
            eprintln!(
                "[NOTE]: Attempting to use {} to connect to: \"{}\"",
                conn_type,
                selected_network.get_public_name()
            );
        }
    }

    let res = match connect_via {
        WifiConnectionType::Netctl => connect_via_netctl(options, interface, selected_network),
        WifiConnectionType::Nmcli => {
            connect_via_networkmanager(options, selected_network, encryption_key)
        }
        WifiConnectionType::Print => {
            let essid = selected_network.get_public_name();
            // TODO: integration tests to ensure this happens
            println!("{}", essid);
            Ok(ConnectionResult {
                connection_type: connect_via.clone(),
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
                    conn_type,
                    selected_network.get_public_name()
                );
            }
            WifiConnectionType::Print => {}
            WifiConnectionType::Netctl | WifiConnectionType::Nmcli => {
                eprintln!(
                    "[NOTE]: Successfully connected to: \"{}\"",
                    selected_network.get_public_name()
                );
            }
        }
    }

    res
}

// TODO: test service-switching behavior in VM integration test
// TODO: make work for wired
fn manage_services<O>(options: &O, interface: &WifiIPInterface) -> Result<(), RuwiError>
where
    O: Global + Wifi + WifiConnect,
{
    let scan_service = options.get_scan_type().get_service(Some(interface));
    let connect_service = options.get_connect_via().get_service(Some(interface));

    if scan_service != connect_service {
        scan_service.stop(options)?;
    }

    connect_service.start(options)?;

    Ok(())
}

fn connect_via_netctl<O>(
    options: &O,
    interface: &WifiIPInterface,
    selected_network: &AnnotatedWirelessNetwork,
) -> Result<ConnectionResult, RuwiError>
where
    O: Global,
{
    if options.get_dry_run() {
        return Ok(ConnectionResult {
            connection_type: WifiConnectionType::Netctl,
        });
    }
    interface.bring_down(options)?;

    let netctl_identifier = NetctlIdentifier::from(selected_network);
    let netctl_switch_to_res = netctl_switch_to(options, &netctl_identifier);

    netctl_switch_to_res.map(|_| ConnectionResult {
            connection_type: WifiConnectionType::Netctl,
        })
}

fn connect_via_networkmanager<O>(
    options: &O,
    selected_network: &AnnotatedWirelessNetwork,
    encryption_key: &Option<String>,
) -> Result<ConnectionResult, RuwiError>
where
    O: Global,
{
    // TODO: see if interface needs to be down
    //bring_interface_down(options)?;

    if options.get_dry_run() {
        return Ok(ConnectionResult {
            connection_type: WifiConnectionType::Nmcli,
        });
    }

    // Refresh NetworkManager's list of known networks, otherwise the connect will
    // fail if we've scanned using another method.
    SystemCommandRunner::new(options, "nmcli", &["device", "wifi", "list"])
        .run_command_status_dumb();

    let args = vec![
        "device",
        "wifi",
        "connect",
        selected_network.get_public_name(),
    ];
    let args = if let Some(pw) = encryption_key {
        let pw_args = vec!["password", pw];
        args.into_iter().chain(pw_args).collect()
    } else {
        args
    };

    SystemCommandRunner::new(options, "nmcli", &args)
        .run_command_pass(
            RuwiErrorKind::FailedToConnectViaNetworkManager,
            "Failed to connect to \"{}\" using nmcli!",
        )
        .map(|_| ConnectionResult {
            connection_type: WifiConnectionType::Nmcli,
        })
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
