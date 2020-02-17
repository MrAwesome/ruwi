// It seems very reasonable for options to be named ...Options
#![allow(clippy::module_name_repetitions)]
// For strum macros:
#![allow(clippy::default_trait_access)]
#![allow(clippy::used_underscore_binding)]

pub(crate) mod command;
pub mod interfaces;
pub(crate) mod wifi;

use crate::enums::*;
use crate::options::interfaces::*;

use typed_builder::TypedBuilder;
pub static PROG_NAME: &str = "ruwi";

#[derive(Debug, Clone, TypedBuilder)]
pub struct GlobalOptions {
    #[builder(default = true)]
    debug: bool,
    #[builder(default = true)]
    dry_run: bool,
    #[builder(default)]
    selection_method: SelectionMethod,
}

impl Global for GlobalOptions {
    fn d(&self) -> bool {
        self.get_debug()
    }
    fn get_debug(&self) -> bool {
        self.debug
    }
    fn get_dry_run(&self) -> bool {
        self.dry_run
    }
    fn get_selection_method(&self) -> &SelectionMethod {
        &self.selection_method
    }
}

impl Default for GlobalOptions {
    fn default() -> Self {
        Self {
            debug: false,
            selection_method: SelectionMethod::default(),
            #[cfg(not(test))]
            dry_run: false,
            #[cfg(test)]
            dry_run: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BluetoothCommandOptions {}
