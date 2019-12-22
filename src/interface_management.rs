use crate::run_commands::*;
use crate::structs::*;

use strum_macros::{AsStaticStr, Display, EnumIter, EnumString};

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
enum InterfaceState {
    UP,
    DOWN,
}

fn bring_interface(
    options: &Options,
    interface_state: &InterfaceState,
    err_kind: RuwiErrorKind,
) -> Result<(), RuwiError> {
    let if_name = &options.interface;
    let if_state = interface_state.to_string();
    let cmd = "ip";
    let cmd_args = &["link", "set", "dev", if_name, &if_state];
    if !options.dry_run {
        run_command_pass_stdout(
            options.debug,
            cmd,
            cmd_args,
            err_kind,
            &format!(
                "Failed to bring interface {} {} with `{} {}`. Try running {} with `sudo`.",
                if_name,
                if_state,
                cmd,
                cmd_args.join(" "),
                PROG_NAME
            ),
        )?;
    }

    Ok(())
}

pub(crate) fn bring_interface_up(options: &Options) -> Result<(), RuwiError> {
    bring_interface(
        options,
        &InterfaceState::UP,
        RuwiErrorKind::FailedToBringInterfaceUp,
    )
}

pub(crate) fn bring_interface_down(options: &Options) -> Result<(), RuwiError> {
    bring_interface(
        options,
        &InterfaceState::DOWN,
        RuwiErrorKind::FailedToBringInterfaceUp,
    )
}
