use crate::run_commands::*;
use crate::errors::*;
use crate::options::interfaces::Global;
use crate::interface_management::ip_interfaces::*;
use crate::enums::RawInterfaceConnectionType;

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

    pub(crate) fn connect(&self) -> Result<(), RuwiError> 
    {
        match self.connect_via {
            RawInterfaceConnectionType::Dhcpcd => self.dhcpcd_connect(),
            _ => unimplemented!("Other connection types not yet implemented!"),
        }
    }

    fn dhcpcd_connect(&self) -> Result<(), RuwiError> {
        run_command_pass(
            self.options, 
            "dhcpcd",
            &[self.interface.get_ifname()],
            RuwiErrorKind::FailedToRawConnectViaDhcpcd,
            &format!("Failed to connect on \"{}\" using dhcpcd!", self.interface.get_ifname())
        )
    }
}
