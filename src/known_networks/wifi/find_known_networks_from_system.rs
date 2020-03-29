use crate::enums::*;
use crate::errors::*;
use crate::options::interfaces::*;
use crate::rerr;
#[cfg(not(test))]
use crate::run_commands::SystemCommandRunner;

#[cfg(not(test))]
use std::os::unix::ffi::OsStrExt;
#[cfg(not(test))]
use std::fs::{read_dir, DirEntry, File};
use std::io;
#[cfg(not(test))]
use std::io::prelude::*;
#[cfg(not(test))]
use std::path::Path;
#[cfg(not(test))]
use unescape::unescape;

use super::{WifiKnownNetworks, UnparsedKnownNetworkNamesAndIdentifiers};

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
        WifiConnectionType::Netctl => find_known_netctl_networks()
            .map_err(|_e| rerr!(RuwiErrorKind::KnownNetworksFetchError, "Failed to fetch known network names for netctl! Does /etc/netctl/ exist? Run with `-d` for more info."))?,
        WifiConnectionType::Nmcli => find_known_networkmanager_networks(options)?,
        WifiConnectionType::None | WifiConnectionType::Print => vec![],
    };

    if options.d() {
        dbg![&known_network_list_with_duplicates];
    }

    Ok(WifiKnownNetworks::new(known_network_list_with_duplicates))
}

#[cfg(test)]
fn find_known_networkmanager_networks<O>(_options: &O) -> Result<UnparsedKnownNetworkNamesAndIdentifiers, RuwiError>
where
    O: Global,
{
    Ok(vec![])
}

#[cfg(not(test))]
fn find_known_networkmanager_networks<O>(options: &O) -> Result<UnparsedKnownNetworkNamesAndIdentifiers, RuwiError>
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
fn find_known_netctl_networks() -> io::Result<UnparsedKnownNetworkNamesAndIdentifiers> {
    Ok(vec![])
}

#[cfg(not(test))]
fn find_known_netctl_networks() -> io::Result<UnparsedKnownNetworkNamesAndIdentifiers> {
    let netctl_path = Path::new("/etc/netctl");
    if netctl_path.is_dir() {
        // TODO: Use tokio/etc to asynchronously read from these files
        let known_essids = read_dir(netctl_path)?
            .filter_map(|entry| get_essid_from_netctl_config_file(entry).ok())
            .filter_map(|essid_entry| {
                if let Some((essid, identifier)) = essid_entry {
                    let escaped_essid = unescape(&essid).unwrap_or(essid);
                    Some((escaped_essid, NetworkServiceIdentifier::Netctl(identifier)))
                } else {
                    None
                }
            })
            .collect();

        Ok(known_essids)
    } else {
        Ok(vec![])
    }
}

#[cfg(not(test))]
fn get_essid_from_netctl_config_file(entry: io::Result<DirEntry>) -> io::Result<Option<(String, String)>> {
    let entry = entry?;
    let path = entry.path();
    if path.is_file() {
        let mut contents = String::new();
        let mut f = File::open(&path)?;
        f.read_to_string(&mut contents)?;
        if let Some(essid) = get_essid_from_netctl_config_text(&contents) {
            let file_name = path.file_name();
            if let Some(osstr_name) = file_name {
                return Ok(Some((essid, String::from_utf8_lossy((*osstr_name).as_bytes()).to_string())));
            }
        }
    }


    Ok(None)
}

fn get_essid_from_netctl_config_text(contents: &str) -> Option<String> {
    contents.lines().find_map(|line| {
        if line.starts_with("ESSID=") {
            Some(
                line.trim_start_matches("ESSID=")
                    .trim_start_matches('\'')
                    .trim_start_matches('"')
                    .trim_end_matches('\'')
                    .trim_end_matches('"')
                    .into(),
            )
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_essid_from_netctl_config() {
        let essid = "fart".to_string();
        let contents = format!(
            "
DOES
NOT=MATTER
# really does not matter 
ESSID={}",
            essid
        );
        let res = get_essid_from_netctl_config_text(&contents).unwrap();
        assert_eq![essid, res];
    }

    #[test]
    fn test_get_essid_with_single_quotes_from_netctl_config() {
        let essid = "fart".to_string();
        let contents = format!(
            "
DOES
NOT=MATTER
# really does not matter 
ESSID='{}'",
            essid
        );
        let res = get_essid_from_netctl_config_text(&contents).unwrap();
        assert_eq![essid, res];
    }

    #[test]
    fn test_get_essid_with_double_quotes_from_netctl_config() {
        let essid = "fart".to_string();
        let contents = format!(
            "
DOES
NOT=MATTER
# really does not matter 
ESSID=\"{}\"",
            essid
        );
        let res = get_essid_from_netctl_config_text(&contents).unwrap();
        assert_eq![essid, res];
    }

    #[test]
    fn test_get_no_essid_from_netctl_config_text() {
        let contents = "
DOES
NOT=MATTER
# la la la la la
JKLFDJSKLFJDSLKJFD
SA T N ANNNANBLAH
";
        let res = get_essid_from_netctl_config_text(&contents);
        assert![res.is_none()];
    }
}
