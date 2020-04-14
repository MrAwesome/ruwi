use super::*;

#[cfg(not(test))]
use std::fs::{read_dir, DirEntry, File};
#[cfg(not(test))]
use std::io;
#[cfg(not(test))]
use std::io::prelude::*;
#[cfg(not(test))]
use std::path::Path;

#[cfg(not(test))]
use crate::utils::convert_osstr_to_string;

const ESSID_TOKEN: &str = "ESSID=";

impl<'a, O: Global> NetctlConfigHandler<'a, O> {}

impl<'a> TryFrom<&NetctlRawConfig<'a>> for NetctlRawParsedFields {
    type Error = RuwiError;

    fn try_from(f: &NetctlRawConfig) -> Result<Self, RuwiError> {
        // TODO: here, or in a helper function, grab the fields you need from a netctl config blob
        unimplemented!()
    }
}

#[cfg(test)]
pub(super) fn read_all_netctl_config_files<'a>(
    netctl_path_name: &'a str,
) -> Result<Vec<NetctlRawConfig<'a>>, RuwiError> {
    unimplemented!()
}

#[cfg(not(test))]
pub(super) fn read_all_netctl_config_files<'a>(
    netctl_path_name: &'a str,
) -> Result<Vec<NetctlRawConfig<'a>>, RuwiError> {
    let netctl_path = Path::new(netctl_path_name);
    if netctl_path.is_dir() {
        // TODO: Use tokio/etc to asynchronously read from these files
        let dir_entries = read_dir(netctl_path).map_err(|e| {
            rerr!(
                RuwiErrorKind::ErrorReadingNetctlDir,
                format!(
                    "Failed trying to read contents of netctl dir: {}",
                    netctl_path_name
                ),
                "OS_ERR" => e
            )
        })?;

        let mut found_files = vec![];
        for dir_entry_res in dir_entries {
            let filename_and_contents = read_file_contents(dir_entry_res).map_err(|e| {
                rerr!(
                    RuwiErrorKind::ErrorReadingNetctlDir,
                    format!(
                        "Failed trying to read contents of netctl config in: {}",
                        netctl_path_name,
                    ),
                    "OS_ERR" => e
                )
            })?;
            if let Some(entry) = filename_and_contents {
                found_files.push(entry);
            }
        }

        Ok(found_files
            .iter()
            .map(|(file_name, file_contents)| {
                NetctlRawConfig::new(
                    NetctlIdentifier::new(file_name),
                    NetctlRawConfigContents::new(file_contents),
                    netctl_path_name,
                )
            })
            .collect())
    } else {
        Err(rerr!(
            RuwiErrorKind::InvalidNetctlPath,
            format!(
                "Given netctl path is not a valid directory: {}",
                netctl_path_name
            ),
        ))
    }
}

type FileName = String;
type FileContents = String;

#[cfg(not(test))]
fn read_file_contents(
    entry_res: io::Result<DirEntry>,
) -> io::Result<Option<(FileName, FileContents)>> {
    let entry = entry_res?;
    let path = entry.path();
    if path.is_file() {
        let mut contents = String::new();
        let mut f = File::open(&path)?;
        f.read_to_string(&mut contents)?;

        let file_name = path.file_name();
        if let Some(osstr_name) = file_name {
            return Ok(Some((convert_osstr_to_string(osstr_name), contents)));
        }
    }
    Ok(None)
}

// .filter_map(|entry| get_essid_from_netctl_config_file(entry).ok())
// .filter_map(|essid_entry| {
//     if let Some((essid, identifier)) = essid_entry {
//         let escaped_essid = unescape(&essid).unwrap_or(essid);
//         Some((escaped_essid, NetworkServiceIdentifier::Netctl(identifier)))
//     } else {
//         None
//     }
// })

// TODO: make this more generic, get_field_from_netctl_config
fn get_field_from_netctl_config_text(contents: &str, token: &str) -> Option<String> {
    contents.lines().find_map(|line| {
        if line.starts_with(token) {
            let value = line
                .trim_start_matches(token)
                .trim_start_matches('\'')
                .trim_start_matches('"')
                .trim_end_matches('\'')
                .trim_end_matches('"')
                .to_string();
            Some(value)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    //use super::*;
}
