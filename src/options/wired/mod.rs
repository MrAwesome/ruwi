pub(crate) mod connect;

use crate::enums::*;
use crate::interface_management::ip_interfaces::*;
use crate::options::interfaces::*;
use crate::options::GlobalOptions;
use typed_builder::TypedBuilder;

// TODO: connect with netctl
// TODO: connect with netctl (support encrypted connections?)
// TODO: connect with dhcpcd
// TODO: connect with dhclient

#[derive(Debug, Clone, TypedBuilder)]
pub struct WiredOptions {
    globals: GlobalOptions,
    interface: WiredIPInterface,
}

impl Default for WiredOptions {
    fn default() -> Self {
        Self {
            globals: GlobalOptions::default(),
            interface: WiredIPInterface::default(),
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
}
