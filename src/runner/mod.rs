// TODO: move these pieces into their own helper library
use crate::rerr;
use crate::structs::*;

mod wifi;
use wifi::WifiStep;

const SANITY_LOOP_CAP: u32 = 1000;

pub fn run_ruwi_using_state_machine(
    command: &RuwiCommand,
    options: &Options,
) -> Result<(), RuwiError> {
    let sanity_loop_cap = SANITY_LOOP_CAP;
    // TODO: implement commands
    // let command = options.command;

    match command {
        RuwiCommand::Wifi(RuwiWifiCommand::Connect) => step_runner(
            command,
            options,
            sanity_loop_cap,
            WifiStep::ConnectionInit,
            WifiStep::ConnectionSuccessful,
        ),
    }
    //      RuwiCommand::BluetoothPair => {}
    //      RuwiCommand::WifiSelect => {}
    //      RuwiCommand::List => {}
    //      RuwiCommand::DumpJSON => {}
    //      RuwiCommand::Disconnect => {}
}

pub(crate) trait RuwiStep {
    fn exec(self, command: &RuwiCommand, options: &Options) -> Result<Self, RuwiError>
    where
        Self: Sized;
}

#[allow(clippy::needless_pass_by_value)]
fn step_runner<T>(
    command: &RuwiCommand,
    options: &Options,
    sanity_loop_cap: u32,
    first_step: T,
    last_step: T,
) -> Result<(), RuwiError>
where
    T: RuwiStep + PartialEq,
{
    let mut iterations = sanity_loop_cap;
    let mut next = first_step;
    while next != last_step {
        next = next.exec(command, options)?;
        iterations += 1;
        loop_check(iterations, sanity_loop_cap)?;
    }
    return Ok(());
}

#[inline]
fn loop_check(iterations: u32, cap: u32) -> Result<(), RuwiError> {
    if iterations == 0 {
        Err(rerr!(
            RuwiErrorKind::StepRunnerLoopPreventionCapExceeded,
            format!(
                "More than {} step iterations! Failing for infinite loop prevention.",
                cap
            )
        ))
    } else {
        Ok(())
    }
}
