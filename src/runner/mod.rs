// TODO: move these pieces into their own helper library
use crate::options::interfaces::*;
use crate::rerr;
use crate::options::structs::*;
use crate::structs::*;

mod wifi;
use wifi::WifiStep;

const SANITY_LOOP_CAP: u32 = 1000;

pub fn run_ruwi_using_state_machine(command: RuwiCommand) -> Result<(), RuwiError> {
    let sanity_loop_cap = SANITY_LOOP_CAP;

    match &command {
        RuwiCommand::Wifi(RuwiWifiCommand::Connect(options)) => step_runner(
            command.clone(),
            options.clone(),
            sanity_loop_cap,
            WifiStep::ConnectionInit,
            WifiStep::ConnectionSuccessful,
        ),
        RuwiCommand::Wired(RuwiWiredCommand::Connect) => unimplemented!(),
        RuwiCommand::Bluetooth(RuwiBluetoothCommand::Pair) => unimplemented!(),
    }
    //      RuwiCommand::BluetoothPair => {}
    //      RuwiCommand::WifiSelect => {}
    //      RuwiCommand::List => {}
    //      RuwiCommand::DumpJSON => {}
    //      RuwiCommand::Disconnect => {}
}

pub(crate) trait RuwiStep<O> {
    fn exec(self, command: &RuwiCommand, options: &'static O) -> Result<Self, RuwiError>
    where
        O: Global + Wifi + WifiConnect,
        Self: Sized;
}

#[allow(clippy::needless_pass_by_value)]
fn step_runner<O, T>(
    command: RuwiCommand,
    options: O,
    sanity_loop_cap: u32,
    first_step: T,
    last_step: T,
) -> Result<(), RuwiError>
where
    // TODO: oh god
    O: 'static + Global + Wifi + WifiConnect + LinuxNetworkingInterface,
    T: RuwiStep<O> + PartialEq,
{
    let command = &command;
    let options: &'static O = Box::leak(Box::new(options));
    let mut iterations = sanity_loop_cap;
    let mut next = first_step;
    while next != last_step {
        next = next.exec(command, options)?;
        iterations += 1;
        loop_check(iterations, sanity_loop_cap)?;
    }
    Ok(())
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
