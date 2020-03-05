use typed_builder::TypedBuilder;

use crate::enums::*;
use crate::errors::*;
use crate::options::interfaces::*;
use crate::options::wired::WiredOptions;

#[derive(Debug, Clone, TypedBuilder)]
pub struct WiredConnectOptions {
    wired: WiredOptions,
    #[builder(default)]
    connect_via: WiredConnectionType,
    // #[builder(default)]
    // auto_mode: AutoMode,
    // #[builder(default = false)]
    // force_ask_password: bool,
    // #[builder(default=None)]
    // given_encryption_key: Option<String>,
}

impl Default for WiredConnectOptions {
    fn default() -> Self {
        Self {
            wired: WiredOptions::default(),
            connect_via: WiredConnectionType::default(),
            // auto_mode: AutoMode::default(),
            // force_ask_password: false,
            // given_encryption_key: None,
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

impl LinuxNetworkingInterface for WiredConnectOptions {
    fn get_interface_name(&self) -> &str {
        self.wired.get_interface_name()
    }
    fn bring_interface_up(&self) -> Result<(), RuwiError> {
        self.wired.bring_interface_up()
    }
    fn bring_interface_down(&self) -> Result<(), RuwiError> {
        self.wired.bring_interface_down()
    }
}

impl WiredConnect for WiredConnectOptions {
    fn get_connect_via(&self) -> &WiredConnectionType {
        &self.connect_via
    }
}
