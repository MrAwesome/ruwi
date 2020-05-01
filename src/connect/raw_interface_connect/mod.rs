use crate::prelude::*;
use crate::enums::NetworkingService;
use crate::enums::WiredConnectionType;
use crate::interface_management::ip_interfaces::{LinuxIPInterface, WiredIPInterface};
use crate::netctl::utils::netctl_switch_to;
use crate::netctl::NetctlIdentifier;
use crate::run_commands::SystemCommandRunner;

// TODO: connect with netctl (support encrypted connections?)

pub(crate) struct RawInterfaceConnector<'a, O: Global + Wired + WiredConnect> {
    options: &'a O,
    interface: &'a WiredIPInterface,
}

impl<'a, O: Global + Wired + WiredConnect> RawInterfaceConnector<'a, O> {
    pub(crate) fn new(options: &'a O, interface: &'a WiredIPInterface) -> Self {
        Self { options, interface }
    }

    // TODO: support profile connections with nmcli? It doesn't seem to support them.
    pub(crate) fn connect(&self, network: &AnnotatedWiredNetwork) -> Result<(), RuwiError> {
        match self.options.get_connect_via() {
            WiredConnectionType::Dhcpcd => self.dhcpcd_connect(),
            WiredConnectionType::Dhclient => self.dhclient_connect(),
            WiredConnectionType::Nmcli => self.nmcli_connect(),
            WiredConnectionType::Netctl => self.netctl_connect(network),
        }
    }

    fn dhcpcd_connect(&self) -> Result<(), RuwiError> {
        SystemCommandRunner::new(self.options, "dhcpcd", &[self.interface.get_ifname()])
            .run_command_pass(
                RuwiErrorKind::FailedToRawConnectViaDhcpcd,
                &format!(
                    "Failed to connect on \"{}\" using dhcpcd!",
                    self.interface.get_ifname()
                ),
            )
    }

    fn dhclient_connect(&self) -> Result<(), RuwiError> {
        SystemCommandRunner::new(self.options, "dhclient", &[self.interface.get_ifname()])
            .run_command_pass(
                RuwiErrorKind::FailedToRawConnectViaDhclient,
                &format!(
                    "Failed to connect on \"{}\" using dhclient!",
                    self.interface.get_ifname()
                ),
            )
    }

    fn nmcli_connect(&self) -> Result<(), RuwiError> {
        NetworkingService::NetworkManager.start(self.options)?;
        SystemCommandRunner::new(
            self.options,
            "nmcli",
            &["device", "connect", self.interface.get_ifname()],
        )
        .run_command_pass(
            RuwiErrorKind::FailedToRawConnectViaNmcli,
            &format!(
                "Failed to connect on \"{}\" using nmcli!",
                self.interface.get_ifname()
            ),
        )
    }

    fn netctl_connect(&self, network: &AnnotatedWiredNetwork) -> Result<(), RuwiError> {
        NetworkingService::Netctl.start(self.options)?;

        let identifier = NetctlIdentifier::from(network);
        netctl_switch_to(self.options, &identifier)
    }
}
