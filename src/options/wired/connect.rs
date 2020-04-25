use typed_builder::TypedBuilder;

use crate::enums::*;
use crate::options::interfaces::*;
use crate::options::wired::WiredOptions;

#[derive(Debug, Clone, TypedBuilder)]
pub struct WiredConnectOptions {
    wired: WiredOptions,
    #[builder(default)]
    connect_via: RawInterfaceConnectionType,
    #[builder(default)]
    given_profile_name: Option<String>,
}

impl Default for WiredConnectOptions {
    fn default() -> Self {
        Self {
            wired: WiredOptions::default(),
            connect_via: RawInterfaceConnectionType::default(),
            given_profile_name: None,
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
    fn pretend_to_be_root(&self) -> bool {
        self.wired.pretend_to_be_root()
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

    fn get_given_profile_name(&self) -> &Option<String> {
        &self.given_profile_name
    }
}
