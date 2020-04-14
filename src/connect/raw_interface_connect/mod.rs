use crate::enums::NetworkingService;
use crate::enums::RawInterfaceConnectionType;
use crate::errors::*;
use crate::interface_management::ip_interfaces::*;
use crate::options::interfaces::*;
use crate::run_commands::SystemCommandRunner;

// TODO: connect with netctl
// TODO: connect with netctl (support encrypted connections?)

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
            RawInterfaceConnectionType::Netctl => self.netctl_connect(),
        }
    }

    fn todo() {
        "implement raw interface connect for netctl - create config, just with no encryption info or essid";
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
    fn netctl_connect(&self) -> Result<(), RuwiError> {
        NetworkingService::Netctl.start(self.options)?;
        // TODO: when given interface, look for netctl profiles using given interface, or create one
        // TODO: look for "Connection=ethernet" instead of ESSID
        // TODO: give a selector for them? or just use the first?
        // TODO: cmdline option for specifying netctl profile to connect to? at that point should
        // people just use netctl?

        todo!("netctl wired connections aren't implemented yet - specify another type with `-c`, like `-c dhclient`");
    }
}
