use super::structs::*;
use super::NetctlIdentifier;

use std::fs::File;
use std::io;
use std::io::Write;

// TODO: this and check_connection_type should not use RuwiError, but something more specific (see parse errors)
pub(super) fn check_for_field<'a>(
    field: &'a Option<String>,
    filename: &NetctlIdentifier,
    field_name: &str,
) -> Result<&'a str, NetctlParseError> {
    match field {
        Some(val) => Ok(&val),
        None => Err(NetctlParseError::MissingFieldInNetctlConfig(format!(
            "Required field \"{}\" was not found in netctl config \"{}\"!",
            field_name, filename
        ))),
    }
}

pub(super) fn check_connection_type(
    expected: NetctlConnectionType,
    actual: NetctlConnectionType,
) -> Result<(), NetctlParseError> {
    if expected == actual {
        Ok(())
    } else {
        Err(NetctlParseError::IncorrectConnectionType)
    }
}

pub(super) fn write_to_netctl_config(fullpath: &str, contents: &str) -> io::Result<()> {
    File::create(&fullpath)?.write_all(contents.as_bytes())
}
