// TODO: determine if things in this file should be unit tested, or just integration tested.
#[cfg(not(test))]
pub(super) mod reader_implementation {
    use super::super::{NetctlRawConfig, RuwiError, RuwiErrorKind, rerr};

    use std::fs::{read_dir, DirEntry, File};
    use std::io;
    use std::io::prelude::*;
    use std::path::Path;

    use crate::utils::convert_osstr_to_string;

    pub(in super::super) fn read_all_netctl_config_files<'a>(
        netctl_path_name: &'a str,
    ) -> Result<Vec<NetctlRawConfig<'a>>, RuwiError> {
        let files = read_all_netctl_config_files_impl(netctl_path_name)?;
        Ok(
            files.iter()
            .map(|(file_name, file_contents)| {
                NetctlRawConfig::builder()
                    .identifier(file_name)
                    .contents(file_contents)
                    .location(netctl_path_name)
                    .build()
            })
            .collect())
    }

    fn read_all_netctl_config_files_impl(
        netctl_path_name: &str,
    ) -> Result<Vec<(FileName, FileContents)>, RuwiError> {
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

            Ok(found_files)
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
}

#[cfg(test)]
mod tests {
    //use super::*;
}
