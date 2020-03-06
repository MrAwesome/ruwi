use crate::enums::*;
use crate::errors::*;
use crate::options::interfaces::*;
use crate::options::wired::connect::WiredConnectOptions;
use crate::runner::Runner;
use crate::structs::*;

impl Runner for WiredConnectOptions {
    fn run(&self) -> Result<(), RuwiError> {

        Ok(())
    }
}
