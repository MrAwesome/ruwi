use crate::rerr;
#[cfg(not(test))]
use crate::run_commands::*;
use crate::structs::*;
#[cfg(not(test))]
use std::collections::HashSet;
use std::error::Error;
#[cfg(not(test))]
use std::fs::{read_dir, DirEntry, File};
use std::io;
#[cfg(not(test))]
use std::io::prelude::*;
#[cfg(not(test))]
use std::path::Path;
#[cfg(not(test))]
use unescape::unescape;

pub(crate) fn find_known_network_names(options: &Options) -> Result<KnownNetworkNames, RuwiError> {
    if options.dry_run {
        return Ok(KnownNetworkNames::default());
    }

    let known_network_names = match options.connect_via {
        ConnectionType::Netctl => find_known_netctl_networks()
            .map_err(|e| rerr!(RuwiErrorKind::KnownNetworksFetchError, e.description())),
        ConnectionType::NetworkManager => find_known_networkmanager_networks(&options),
        ConnectionType::None | ConnectionType::Print => Ok(KnownNetworkNames::default()),
    };

    if options.debug {
        dbg![&known_network_names];
    }

    known_network_names
}

#[cfg(test)]
fn find_known_networkmanager_networks(_options: &Options) -> Result<KnownNetworkNames, RuwiError> {
    Ok(KnownNetworkNames::default())
}

#[cfg(not(test))]
fn find_known_networkmanager_networks(options: &Options) -> Result<KnownNetworkNames, RuwiError> {
    let cmd_output = run_command_pass_stdout(
        options.debug,
        "nmcli",
        &["-g", "NAME", "connection", "show"],
        RuwiErrorKind::FailedToListKnownNetworksWithNetworkManager,
        "Failed to list known networks with NetworkManager. Try running `nmcli -g NAME connection show`.",
    )?;
    let known_names = cmd_output
        .lines()
        .map(|x| x.into())
        .collect::<HashSet<String>>();
    Ok(KnownNetworkNames(known_names))
}


#[cfg(test)]
fn find_known_netctl_networks() -> io::Result<KnownNetworkNames> {
    Ok(KnownNetworkNames::default())
}

#[cfg(not(test))]
fn find_known_netctl_networks() -> io::Result<KnownNetworkNames> {
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
            // TODO: unit test that unescape happens
            .collect::<HashSet<String>>();

        Ok(KnownNetworkNames(known_essids))
    } else {
        Ok(KnownNetworkNames::default())
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
