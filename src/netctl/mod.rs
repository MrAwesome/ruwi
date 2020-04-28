mod config_handler;
mod identifiers;
pub(crate) mod utils;

const DEFAULT_NETCTL_CFG_DIR: &str = "/etc/netctl/";

pub(crate) use config_handler::NetctlConfigHandler;
pub(crate) use identifiers::NetctlIdentifier;
