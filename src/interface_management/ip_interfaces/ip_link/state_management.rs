use crate::errors::*;
use crate::options::interfaces::*;
use crate::options::PROG_NAME;
use crate::run_commands::SystemCommandRunner;

use strum_macros::{AsStaticStr, Display, EnumIter, EnumString};

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
enum InterfaceState {
    UP,
    DOWN,
}

fn bring_interface<O>(
    options: &O,
    interface_name: &str,
    interface_state: &InterfaceState,
    err_kind: RuwiErrorKind,
) -> Result<(), RuwiError>
where
    O: Global,
{
    let cmd = "ip";
    let cmd_args = &[
        "link",
        "set",
        "dev",
        interface_name,
        &interface_state.to_string(),
    ];
    if !options.get_dry_run() {
        SystemCommandRunner::new(
            options,
            cmd,
            cmd_args,
        ).run_command_pass_stdout(
            err_kind,
            &format!(
                "Failed to bring interface {} {} with `{} {}`. Try running {} with `sudo`.",
                interface_name,
                interface_state,
                cmd,
                cmd_args.join(" "),
                PROG_NAME
            ),
        )?;
    }

    Ok(())
}

pub(crate) fn bring_up<O>(
    options: &O,
    interface_name: &str,
) -> Result<(), RuwiError>
where
    O: Global,
{
    bring_interface(
        options,
        interface_name,
        &InterfaceState::UP,
        RuwiErrorKind::FailedToBringLinuxNetworkingInterfaceUp,
    )
}

pub(crate) fn bring_down<O>(
    options: &O,
    interface_name: &str,
) -> Result<(), RuwiError>
where
    O: Global,
{
    bring_interface(
        options,
        interface_name,
        &InterfaceState::DOWN,
        RuwiErrorKind::FailedToBringLinuxNetworkingInterfaceDown,
    )
}
