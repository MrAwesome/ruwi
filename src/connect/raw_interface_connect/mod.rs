use crate::common::*;
use crate::enums::NetworkingService;
use crate::enums::RawInterfaceConnectionType;
use crate::interface_management::ip_interfaces::*;
use crate::netctl::utils::netctl_switch_to;
use crate::netctl::NetctlConfigHandler;
use crate::run_commands::SystemCommandRunner;

// TODO: connect with netctl
// TODO: connect with netctl (support encrypted connections?)

pub(crate) struct RawInterfaceConnector<'a, O: Global> {
    options: &'a O,
    interface: &'a WiredIPInterface,
    connect_via: &'a RawInterfaceConnectionType,
}

impl<'a, O: Global> RawInterfaceConnector<'a, O> {
    pub(crate) fn new(
        options: &'a O,
        interface: &'a WiredIPInterface,
        connect_via: &'a RawInterfaceConnectionType,
    ) -> Self {
        Self {
            options,
            interface,
            connect_via,
        }
    }

    pub(crate) fn connect(&self, network: AnnotatedWiredNetwork) -> Result<(), RuwiError> {
        match self.connect_via {
            RawInterfaceConnectionType::Dhcpcd => self.dhcpcd_connect(),
            RawInterfaceConnectionType::Dhclient => self.dhclient_connect(),
            RawInterfaceConnectionType::Nmcli => self.nmcli_connect(),
            RawInterfaceConnectionType::Netctl => self.netctl_connect(network),
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

    // TODO: unit test? integration test?
    fn netctl_connect(&self, network: AnnotatedWiredNetwork) -> Result<(), RuwiError> {
        NetworkingService::Netctl.start(self.options)?;

        let ifname = self.interface.get_ifname();
        let handler = NetctlConfigHandler::new(self.options);
        let identifiers = handler.get_wired_configs_with_interface(ifname)?;

        // TODO: use selection here if multiple profiles detected?
        if identifiers.len() > 1 {
            eprintln!("[NOTE]: More than one matching netctl profile was found for interface {}. Will use the first. Manually specify the profile you want with `-p <profilename>` if this is not what you want.", ifname);
        }

        let identifier = match identifiers.first() {
            Some(identifier) => identifier.clone(),
            None => {
                eprintln!("[NOTE]: No existing netctl profile found for interface {}. Will create one now.", ifname);
                handler.write_wired_config(self.interface, &network)?
            } //todo!("create the config and return its identifier (maybe check a flag for if we should?)"),
        };

        // TODO: create netctl/connect, use that here and for wifi. don't put it on confighandler, since this just runs an external command. maybe just in utils?
        netctl_switch_to(self.options, identifier)
    }
}
