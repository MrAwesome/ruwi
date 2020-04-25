mod config_handler;
pub(crate) mod utils;

use crate::string_container;

const DEFAULT_NETCTL_CFG_DIR: &str = "/etc/netctl/";

string_container! {NetctlIdentifier}

pub(crate) use config_handler::NetctlConfigHandler;
