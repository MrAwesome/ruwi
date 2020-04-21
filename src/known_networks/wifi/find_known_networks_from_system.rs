use crate::enums::*;
use crate::errors::*;
use crate::options::interfaces::*;
#[cfg(not(test))]
use crate::run_commands::SystemCommandRunner;

#[cfg(not(test))]
use crate::netctl::NetctlConfigHandler;

use super::{WifiKnownNetworks, UnfilteredKnownNetworkNamesAndIdentifiers};

impl WifiKnownNetworks {
    pub(crate) fn find_known_networks_from_system<O>(options: &O) -> Result<WifiKnownNetworks, RuwiError>
    where
        O: Global + Wifi + WifiConnect,
    {
        find_known_networks(options)
    }
}

// TODO: have a trait for known identifiers, have netctl and networkmanager both implement it

// TODO: unit test the logic in this function
fn find_known_networks<O>(options: &O) -> Result<WifiKnownNetworks, RuwiError>
where
    O: Global + Wifi + WifiConnect,
{
    if options.get_dry_run() || options.get_ignore_known() {
        return Ok(WifiKnownNetworks::default());
    }

    let known_network_list_with_duplicates = match options.get_connect_via() {
        WifiConnectionType::Netctl => find_known_netctl_networks(options)?,
        WifiConnectionType::Nmcli => find_known_networkmanager_networks(options)?,
        WifiConnectionType::None | WifiConnectionType::Print => vec![],
    };

    if options.d() {
        dbg![&known_network_list_with_duplicates];
    }

    Ok(WifiKnownNetworks::new(known_network_list_with_duplicates))
}

#[cfg(test)]
fn find_known_networkmanager_networks<O>(_options: &O) -> Result<UnfilteredKnownNetworkNamesAndIdentifiers, RuwiError>
where
    O: Global,
{
    Ok(vec![])
}

#[cfg(not(test))]
fn find_known_networkmanager_networks<O>(options: &O) -> Result<UnfilteredKnownNetworkNamesAndIdentifiers, RuwiError>
where
    O: Global,
{
    NetworkingService::NetworkManager.start(options)?;
    let output = SystemCommandRunner::new( 
        options,
        "nmcli",
        &["-g", "NAME", "connection", "show"],
    ).run_command_pass_stdout(
        RuwiErrorKind::FailedToListKnownNetworksWithNetworkManager,
        "Failed to list known networks with NetworkManager. Try running `nmcli -g NAME connection show`.",
    ).map(|x| x.lines().map(|x| (x.to_string(), NetworkServiceIdentifier::NetworkManager)).collect());
    NetworkingService::NetworkManager.stop(options)?;
    output
}

#[cfg(test)]
fn find_known_netctl_networks<O>(_options: &O) -> Result<UnfilteredKnownNetworkNamesAndIdentifiers, RuwiError> 
where
    O: Global,
{
    Ok(vec![])
}

#[cfg(not(test))]
fn find_known_netctl_networks<O>(options: &O) -> Result<UnfilteredKnownNetworkNamesAndIdentifiers, RuwiError> 
where
    O: Global,
{
    let handler = NetctlConfigHandler::new(options);
    let configs = handler.get_wifi_essids_and_identifiers()?.into_iter().map(|(essid, identifier)| (essid, NetworkServiceIdentifier::Netctl(identifier.to_string()))).collect();
    Ok(configs)
                    //Some((escaped_essid, NetworkServiceIdentifier::Netctl(identifier)))

}
