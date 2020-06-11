use crate::enums::NetworkingService;
use crate::interface_management::ip_interfaces::*;
use crate::options::clear::ClearOptions;
use crate::prelude::*;
use crate::runner::Runner;

impl Runner for ClearOptions {
    fn run(&self) -> Result<(), RuwiError> {
        // NOTE: these can most likely be run in parallel
        eprintln!("[NOTE]: Bringing down all wired interfaces...");
        WiredIPInterface::bring_all_down(self)?;

        eprintln!("[NOTE]: Bringing down all wifi interfaces...");
        WifiIPInterface::bring_all_down(self)?;

        eprintln!("[NOTE]: Stopping all known networking services...");
        NetworkingService::stop_all(self)?;

        Ok(())
    }
}
