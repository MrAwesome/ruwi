use crate::enums::NetworkingService;

use crate::prelude::*;
use crate::options::clear::ClearOptions;
use crate::runner::Runner;

impl Runner for ClearOptions {
    fn run(&self) -> Result<(), RuwiError> {
        NetworkingService::stop_all(self)?;
        let todo = "Take down interfaces";
        let todo = "Don't error if wpa_supplicant isn't running";
        Ok(())
    }
}
