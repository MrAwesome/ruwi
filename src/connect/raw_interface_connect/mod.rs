use crate::enums::NetworkingService;
use crate::enums::RawInterfaceConnectionType;
use crate::errors::*;
use crate::interface_management::ip_interfaces::*;
use crate::options::interfaces::*;
use crate::run_commands::*;

pub(crate) struct RawInterfaceConnector<'a, O: Global, T: LinuxIPInterface> {
    options: &'a O,
    interface: &'a T,
    connect_via: &'a RawInterfaceConnectionType,
}

impl<'a, O: Global, T: LinuxIPInterface> RawInterfaceConnector<'a, O, T> {
    pub(crate) fn new(
        options: &'a O,
        interface: &'a T,
        connect_via: &'a RawInterfaceConnectionType,
    ) -> Self {
        Self {
            options,
            interface,
            connect_via,
        }
    }

    pub(crate) fn connect(&self) -> Result<(), RuwiError> {
        match self.connect_via {
            RawInterfaceConnectionType::Dhcpcd => self.dhcpcd_connect(),
            RawInterfaceConnectionType::Dhclient => self.dhclient_connect(),
            RawInterfaceConnectionType::Nmcli => self.nmcli_connect(),
            _ => unimplemented!("Other connection types not yet implemented!"),
        }
    }

    fn dhcpcd_connect(&self) -> Result<(), RuwiError> {
        run_command_pass(
            self.options,
            "dhcpcd",
            &[self.interface.get_ifname()],
            RuwiErrorKind::FailedToRawConnectViaDhcpcd,
            &format!(
                "Failed to connect on \"{}\" using dhcpcd!",
                self.interface.get_ifname()
            ),
        )
    }

    fn dhclient_connect(&self) -> Result<(), RuwiError> {
        run_command_pass(
            self.options,
            "dhclient",
            &[self.interface.get_ifname()],
            RuwiErrorKind::FailedToRawConnectViaDhclient,
            &format!(
                "Failed to connect on \"{}\" using dhclient!",
                self.interface.get_ifname()
            ),
        )
    }

    fn nmcli_connect(&self) -> Result<(), RuwiError> {
        NetworkingService::Netctl.start(self.options)?;
        run_command_pass(
            self.options,
            "nmcli",
            &["device", "connect", self.interface.get_ifname()],
            RuwiErrorKind::FailedToRawConnectViaNmcli,
            &format!(
                "Failed to connect on \"{}\" using nmcli!",
                self.interface.get_ifname()
            ),
        )
    }
}
