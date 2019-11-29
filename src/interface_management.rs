use crate::run_commands::*;
use crate::structs::*;
use std::error::Error;

use strum_macros::{AsStaticStr, Display, EnumIter, EnumString};

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
enum InterfaceState {
    UP,
    DOWN,
}

fn bring_interface(
    options: &Options,
    interface_state: InterfaceState,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let if_name = &options.interface;
    let if_state = interface_state.to_string();
    run_command_pass_stdout(
        options.debug,
        "ifconfig",
        &[if_name, &if_state],
        &format!(
            "Failed to bring interface {} {} with `ifconfig {} {}`.",
            if_name, if_state, if_name, if_state
        ),
    )?;

    Ok(())
}

pub(crate) fn bring_interface_up(options: &Options) -> Result<(), Box<dyn Error + Send + Sync>> {
    bring_interface(options, InterfaceState::UP)
}

pub(crate) fn bring_interface_down(options: &Options) -> Result<(), Box<dyn Error + Send + Sync>> {
    bring_interface(options, InterfaceState::DOWN)
}
