use typed_builder::TypedBuilder;

use crate::enums::*;
use crate::options::interfaces::*;
use crate::options::wired::WiredOptions;

#[derive(Debug, Clone, TypedBuilder)]
pub struct WiredConnectOptions {
    wired: WiredOptions,
    #[builder(default)]
    connect_via: RawInterfaceConnectionType,
}

impl Default for WiredConnectOptions {
    fn default() -> Self {
        Self {
            wired: WiredOptions::default(),
            connect_via: RawInterfaceConnectionType::default(),
        }
    }
}

impl Global for WiredConnectOptions {
    fn d(&self) -> bool {
        self.get_debug()
    }
    fn get_debug(&self) -> bool {
        self.wired.get_debug()
    }
    fn get_dry_run(&self) -> bool {
        self.wired.get_dry_run()
    }
    fn get_selection_method(&self) -> &SelectionMethod {
        self.wired.get_selection_method()
    }
    fn is_test_or_dry_run(&self) -> bool {
        self.wired.is_test_or_dry_run()
    }
}

impl Wired for WiredConnectOptions {
    fn get_given_interface_name(&self) -> &Option<String> {
        self.wired.get_given_interface_name()
    }
}

impl WiredConnect for WiredConnectOptions {
    fn get_connect_via(&self) -> &RawInterfaceConnectionType {
        &self.connect_via
    }
}
