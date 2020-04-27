mod config_handler;
pub(crate) mod utils;

pub(crate) use config_handler::NetctlConfigHandler;
const DEFAULT_NETCTL_CFG_DIR: &str = "/etc/netctl/";

use crate::string_container;

string_container! {NetctlIdentifier}
