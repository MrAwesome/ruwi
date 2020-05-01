use super::SystemChecksImpl;

use crate::options::traits::Global;
use crate::run_commands::SystemCommandRunner;

pub(crate) struct SystemCheckerReal<'a, O: Global> {
    opts: &'a O
}


impl<'a, O: Global> SystemCheckerReal<'a, O> {
    pub(crate) fn new(opts: &'a O) -> Self {
        Self {
            opts
        }
    }

    fn check_systemd_unit(&self, unit: &str) -> bool {
        SystemCommandRunner::new(
            self.opts,
        "systemctl",
        &["is-active", "--quiet", unit],
        ).run_command_status_dumb()
    }
}

impl<'a, O: Global> SystemChecksImpl for SystemCheckerReal<'a, O> {
    fn check_networkmanager_running(&self) -> bool {
        self.check_systemd_unit("NetworkManager")
    }

    fn check_netctl_running(&self) -> bool {
        self.check_systemd_unit("netctl")
    }

    fn check_netctl_installed(&self) -> bool {
        SystemCommandRunner::new(self.opts, "netctl", &[]).check_command_exists()
    }

    fn check_networkmanager_installed(&self) -> bool {
        SystemCommandRunner::new(self.opts, "NetworkManager", &[]).check_command_exists()
    }

    fn check_dhclient_installed(&self) -> bool {
        SystemCommandRunner::new(self.opts, "dhclient", &[]).check_command_exists()
    }

    fn check_dhcpcd_installed(&self) -> bool {
        SystemCommandRunner::new(self.opts, "dhcpcd", &[]).check_command_exists()
    }
}
