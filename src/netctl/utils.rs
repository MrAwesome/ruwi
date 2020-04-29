use super::NetctlIdentifier;

use crate::prelude::*;
use crate::run_commands::SystemCommandRunner;

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

