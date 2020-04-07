use super::NetctlIdentifier;
use crate::common::*;
use crate::run_commands::SystemCommandRunner;

use std::fs::File;
use std::io;
use std::io::Write;

pub(crate) fn netctl_switch_to<O>(
    options: &O,
    netctl_identifier: NetctlIdentifier,
) -> Result<(), RuwiError>
where
    O: Global,
{
    SystemCommandRunner::new(
        options,
        "netctl",
        &["switch-to", netctl_identifier.as_ref()],
    )
    .run_command_pass(
        RuwiErrorKind::FailedToConnectViaNetctl,
        &format!(
            "Failed to connect to netctl profile \"{}\"!",
            netctl_identifier.as_ref()
        ),
    )
}

pub(super) fn check_for_field<'a>(
    field: &'a Option<String>,
    filename: &NetctlIdentifier,
    field_name: &str,
) -> Result<&'a str, RuwiError> {
    match field {
        Some(val) => Ok(&val),
        None => Err(rerr!(
            RuwiErrorKind::MissingFieldInNetctlConfig,
            format!(
                "Required field \"{}\" was not found in netctl config \"{}\"!",
                field_name, filename
            ),
        ))
    }
}

pub(super) fn write_to_netctl_config(fullpath: &str, contents: &str) -> io::Result<()> {
    File::create(&fullpath)?.write_all(contents.as_bytes())
}
