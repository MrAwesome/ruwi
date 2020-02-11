use crate::errors::*;
use crate::options::interfaces::*;
use crate::rerr;
#[cfg(not(test))]
use crate::run_commands::*;
use crate::structs::*;
#[cfg(not(test))]
use std::fs::{read_dir, DirEntry, File};
use std::io;
#[cfg(not(test))]
use std::io::prelude::*;
#[cfg(not(test))]
use std::path::Path;
#[cfg(not(test))]
use unescape::unescape;

use crate::check_known_identifiers::KnownIdentifiers;

// TODO: unit test the logic in this function
pub(crate) fn find_known_network_names<O>(
    options: &O,
) -> Result<KnownIdentifiers, RuwiError>
where
    O: Global + Wifi + WifiConnect,
{
    if options.get_dry_run() || options.get_ignore_known() {
        return Ok(KnownIdentifiers::default());
    }

    let known_network_names = match options.get_connect_via() {
        WifiConnectionType::Netctl => find_known_netctl_networks()
            .map_err(|_e| rerr!(RuwiErrorKind::KnownNetworksFetchError, "Failed to fetch known network names for netctl! Does /etc/netctl/ exist? Run with `-d` for more info."))?,
        WifiConnectionType::Nmcli => find_known_networkmanager_networks(options)?,
        WifiConnectionType::None | WifiConnectionType::Print => vec![],
    };

    if options.d() {
        dbg![&known_network_names];
    }

    Ok(KnownIdentifiers::new(known_network_names))
}

#[cfg(test)]
fn find_known_networkmanager_networks<O>(_options: &O) -> Result<Vec<String>, RuwiError>
where
    O: Global,
{
    Ok(vec![])
}

#[cfg(not(test))]
fn find_known_networkmanager_networks<O>(options: &O) -> Result<Vec<String>, RuwiError>
where
    O: Global,
{
    run_command_pass_stdout(
        options,
        "nmcli",
        &["-g", "NAME", "connection", "show"],
        RuwiErrorKind::FailedToListKnownNetworksWithNetworkManager,
        "Failed to list known networks with NetworkManager. Try running `nmcli -g NAME connection show`.",
    ).map(|x| x.lines().map(String::from).collect())
}

#[cfg(test)]
fn find_known_netctl_networks() -> io::Result<Vec<String>> {
    Ok(vec![])
}

#[cfg(not(test))]
fn find_known_netctl_networks() -> io::Result<Vec<String>> {
    let netctl_path = Path::new("/etc/netctl");
    if netctl_path.is_dir() {
        // TODO: Use tokio/etc to asynchronously read from these files
        let known_essids = read_dir(netctl_path)?
            .filter_map(|entry| get_essid_from_netctl_config_file(entry).ok())
            .filter_map(|essid_entry| {
                if let Some(essid) = essid_entry {
                    Some(unescape(&essid).unwrap_or(essid))
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
fn get_essid_from_netctl_config_file(entry: io::Result<DirEntry>) -> io::Result<Option<String>> {
    let entry = entry?;
    let path = entry.path();
    if path.is_file() {
        let mut contents = String::new();
        let mut f = File::open(&path)?;
        f.read_to_string(&mut contents)?;
        Ok(get_essid_from_netctl_config_text(&contents))
    } else {
        Ok(None)
    }
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
