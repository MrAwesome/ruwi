pub(super) mod discovery;
pub(super) mod state_management;
use serde_derive::Deserialize;

// A direct representation of what `ip -j link show` gives back to us in JSON.
//
// WiredLinuxIPLinkInterface and WifiLinuxIPLinkInterface are simple type wrappers
// to give us type safety in the conversions into more publicly-visible interface types
// (your dear author was bitten by the stringly-typed get_first_wi{fi,red} functions
// being so visually similar and returning the same type).
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub(super) struct LinuxIPLinkInterface {
    ifname: String,
    link_type: String,
    operstate: OperState,
    flags: Vec<String>,
}

pub(super) struct WifiLinuxIPLinkInterface(LinuxIPLinkInterface);
impl WifiLinuxIPLinkInterface {
    pub(super) fn get_ifname(&self) -> &str {
        self.0.get_ifname()
    }
}

pub(super) struct WiredLinuxIPLinkInterface(LinuxIPLinkInterface);
impl WiredLinuxIPLinkInterface {
    pub(super) fn get_ifname(&self) -> &str {
        self.0.get_ifname()
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(field_identifier)]
enum OperState {
    UP,
    DOWN,
    UNKNOWN,
    Other(String),
}


impl LinuxIPLinkInterface {
    fn _is_up(&self) -> bool {
        self.operstate == OperState::UP || self.flags.iter().any(|x| x == "UP")
    }
    fn _is_down(&self) -> bool {
        !self._is_up()
    }

    pub(crate) fn get_ifname(&self) -> &str {
        &self.ifname
    }

    pub(crate) fn is_wifi(&self) -> bool {
        let ifname = self.get_ifname();
        ifname.starts_with("wlp") || ifname.starts_with("wlan")
    }

    pub(crate) fn is_wired(&self) -> bool {
        let ifname = self.get_ifname();
        ifname.starts_with("enp") || ifname.starts_with("eth")
    }
}
