use crate::prelude::*;
use crate::options::GlobalOptions;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct ClearOptions {
    globals: GlobalOptions,
}

impl Default for ClearOptions {
    fn default() -> Self {
        Self {
            globals: GlobalOptions::default(),
        }
    }
}

impl Global for ClearOptions {
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
