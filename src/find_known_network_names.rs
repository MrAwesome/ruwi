#[cfg(not(test))]
use crate::rerr;
#[cfg(not(test))]
use crate::run_commands::*;
use crate::structs::*;
use std::collections::HashSet;
#[cfg(not(test))]
use std::error::Error;
#[cfg(not(test))]
use std::fs::{read_dir, DirEntry, File};
#[cfg(not(test))]
use std::io;
#[cfg(not(test))]
use std::io::prelude::*;
use std::ops::Deref;
#[cfg(not(test))]
use std::path::Path;
#[cfg(not(test))]
use unescape::unescape;

#[derive(Debug)]
pub struct KnownNetworks(HashSet<String>);

impl Default for KnownNetworks {
    fn default() -> Self {
        KnownNetworks(HashSet::new())
    }
}

impl Deref for KnownNetworks {
    type Target = HashSet<String>;
    fn deref(&self) -> &HashSet<String> {
        &self.0
    }
}

#[cfg(test)]
pub(crate) fn find_known_network_names(_options: Options) -> Result<KnownNetworks, RuwiError> {
    return Ok(KnownNetworks::default());
}

#[cfg(not(test))]
pub(crate) fn find_known_network_names(options: Options) -> Result<KnownNetworks, RuwiError> {
    let known_network_names = match options.connect_via {
        ConnectionType::Netctl => find_known_netctl_networks()
            .map_err(|e| rerr!(RuwiErrorKind::KnownNetworksFetchError, e.description())),
        ConnectionType::NetworkManager => find_known_networkmanager_networks(&options),
        ConnectionType::None | ConnectionType::Print => Ok(KnownNetworks::default()),
    };

    if options.debug {
        dbg![&known_network_names];
    }

    known_network_names
}

#[cfg(not(test))]
fn find_known_networkmanager_networks(options: &Options) -> Result<KnownNetworks, RuwiError> {
    Ok(KnownNetworks(run_command_pass_stdout(
        options.debug,
        "nmcli",
        &["-g", "NAME", "connection", "show"],
        RuwiErrorKind::FailedToListKnownNetworksWithNetworkManager,
        "Failed to list known networks with NetworkManager. Try running `nmcli -g NAME connection show`.",
    )?
    .lines().map(|x| x.into()).collect::<HashSet<String>>()))
}

#[cfg(not(test))]
fn find_known_netctl_networks() -> io::Result<KnownNetworks> {
    let netctl_path = Path::new("/etc/netctl");
    if netctl_path.is_dir() {
        // TODO: Use tokio/etc to asynchronously read from these files
        let known_essids = read_dir(netctl_path)?
            .filter_map(|entry| get_essid_from_netctl_config_file(entry).ok())
            .filter_map(|x| x)
            // TODO: unit test that unescape happens
            .map(|x| unescape(&x).unwrap_or(x))
            .collect::<HashSet<String>>();

        Ok(KnownNetworks(known_essids))
    } else {
        Ok(Default::default())
    }
}

#[cfg(not(test))]
fn get_essid_from_netctl_config_file(entry: io::Result<DirEntry>) -> io::Result<Option<String>> {
    let entry = entry?;
    let path = entry.path();
    if path.is_file() {
        let mut contents = String::new();
        let mut f = File::open(&path)?;
        f.read_to_string(&mut contents)?;
        Ok(get_essid_from_netctl_config_text(contents))
    } else {
        Ok(None)
    }
}

fn get_essid_from_netctl_config_text(contents: String) -> Option<String> {
    contents
        .lines()
        .find(|line| line.starts_with("ESSID="))
        .map(|line| {
            line.trim_start_matches("ESSID=")
                .trim_start_matches('\'')
                .trim_start_matches('"')
                .trim_end_matches('\'')
                .trim_end_matches('"')
                .into()
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
        let res = get_essid_from_netctl_config_text(contents).unwrap();
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
        let res = get_essid_from_netctl_config_text(contents).unwrap();
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
        let res = get_essid_from_netctl_config_text(contents).unwrap();
        assert_eq![essid, res];
    }

    #[test]
    fn test_get_no_essid_from_netctl_config_text() {
        let contents = format!(
            "
DOES
NOT=MATTER
# la la la la la
"
        );
        let res = get_essid_from_netctl_config_text(contents);
        assert![res.is_none()];
    }
}
