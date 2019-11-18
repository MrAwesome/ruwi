use crate::run_commands::*;
use crate::structs::*;
use std::io;
use std::process::{Command, Stdio};

use strum_macros::{AsStaticStr, Display, EnumIter, EnumString};

#[strum(serialize_all = "snake_case")]
#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
enum InterfaceState {
    UP,
    DOWN,
}

fn bring_interface(options: &Options, interface_state: InterfaceState) -> io::Result<()> {
    let output = Command::new("ifconfig")
        .arg(&options.interface)
        .arg(interface_state.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    if options.debug {
        dbg!(&output);
    }

    Ok(())
}

pub(crate) fn bring_interface_up(options: &Options) -> io::Result<()> {
    bring_interface(options, InterfaceState::UP)
}

pub(crate) fn bring_interface_down(options: &Options) -> io::Result<()> {
    bring_interface(options, InterfaceState::DOWN)
}
