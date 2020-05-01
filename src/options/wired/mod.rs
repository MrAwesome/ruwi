pub(crate) mod connect;

use crate::prelude::*;
use crate::options::GlobalOptions;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct WiredOptions {
    globals: GlobalOptions,
    #[builder(default = None)]
    given_interface_name: Option<String>,
}

impl Default for WiredOptions {
    fn default() -> Self {
        Self {
            globals: GlobalOptions::default(),
            given_interface_name: None,
        }
    }
}

impl Global for WiredOptions {
    fn d(&self) -> bool {
        self.get_debug()
    }
    fn get_debug(&self) -> bool {
        self.globals.get_debug()
    }
    fn get_dry_run(&self) -> bool {
        self.globals.get_dry_run()
    }
    fn get_selection_method(&self) -> &SelectionMethod {
        self.globals.get_selection_method()
    }
    fn is_test_or_dry_run(&self) -> bool {
        self.globals.is_test_or_dry_run()
    }
    fn pretend_to_be_root(&self) -> bool {
        self.globals.pretend_to_be_root()
    }
}

impl Wired for WiredOptions {
    fn get_given_interface_name(&self) -> &Option<String> {
        &self.given_interface_name
    }
}
