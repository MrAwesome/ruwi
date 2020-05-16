use super::LinuxIPInterface;
use super::ip_link::{WifiLinuxIPLinkInterface, WiredLinuxIPLinkInterface};

string_container!{WifiIPInterface, WiredIPInterface}

impl Default for WifiIPInterface {
    fn default() -> Self {
        Self::new(super::DRYRUN_FAKE_INTERFACE_NAME)
    }
}


impl LinuxIPInterface for WifiIPInterface {
    type Finder = WifiLinuxIPLinkInterface;

    fn get_ifname(&self) -> &str {
        &self.as_ref()
    }
}

impl Default for WiredIPInterface {
    fn default() -> Self {
        Self::new(super::DRYRUN_FAKE_INTERFACE_NAME)
    }
}

impl LinuxIPInterface for WiredIPInterface {
    type Finder = WiredLinuxIPLinkInterface;

    fn get_ifname(&self) -> &str {
        &self.as_ref()
    }
}
