use crate::options::interfaces::*;
use crate::options::structs::PROG_NAME;
use crate::run_commands::*;
use crate::errors::*;

use strum_macros::{AsStaticStr, Display, EnumIter, EnumString};

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
enum InterfaceState {
    UP,
    DOWN,
}


// TODO: make this work for wired, wifi, and possibly bluetooth (if needed)
fn bring_interface<O>(
    options: &O,
    interface_state: &InterfaceState,
    err_kind: RuwiErrorKind,
) -> Result<(), RuwiError> where O: Global + LinuxNetworkingInterface {
    let if_name = &options.get_interface();
    let if_state = interface_state.to_string();
    let cmd = "ip";
    let cmd_args = &["link", "set", "dev", if_name, &if_state];
    if !options.get_dry_run() {
        run_command_pass_stdout(
            options,
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

pub(crate) fn bring_interface_up<O>(options: &O) -> Result<(), RuwiError> where O: Global + LinuxNetworkingInterface {
    bring_interface(
        options,
        &InterfaceState::UP,
        RuwiErrorKind::FailedToBringInterfaceUp,
    )
}

pub(crate) fn bring_interface_down<O>(options: &O) -> Result<(), RuwiError> where O: Global + LinuxNetworkingInterface {
    bring_interface(
        options,
        &InterfaceState::DOWN,
        RuwiErrorKind::FailedToBringInterfaceUp,
    )
}
