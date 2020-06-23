use crate::interface_management::ip_interfaces::LinuxIPInterface;
use crate::prelude::*;
use crate::run_commands::SystemCommandRunner;

use std::thread;
use std::time::Duration;

const WPA_CONNECT_ERROR: &str = "Error initializing wpa_supplicant to use wpa_cli. Specify another scan method with -s, or add the following to /etc/wpa_supplicant/wpa_supplicant.conf and try again:

ctrl_interface=/run/wpa_supplicant
ctrl_interface_group=wheel
update_config=1

See https://wiki.archlinux.org/index.php/WPA_supplicant#Connecting_with_wpa_cli for more info.";

pub(crate) fn initialize_wpa_supplicant<O, T>(options: &O, interface: &T) -> Result<(), RuwiError>
where
    O: Global,
    T: LinuxIPInterface,
{
    if wpa_ping_success(options) {
        return Ok(());
    } else {
        eprintln!(
            "[NOTE]: wpa_cli was not functioning correctly. Attempting to start it manually."
        );
        let interface_name = interface.get_ifname();
        let supplicant_status = SystemCommandRunner::new(
            options,
            "wpa_supplicant",
            &[
                "-B",
                "-i",
                interface_name,
                "-c",
                "/etc/wpa_supplicant/wpa_supplicant.conf",
            ],
        )
        .run_command_status_dumb();

        if supplicant_status {
            SystemCommandRunner::new(options, "wpa_cli", &["scan"]).run_command_status_dumb();

            eprintln!("[NOTE]: Sleeping to wait for results from wpa_cli. This should only happen when you first start wpa_supplicant. If you aren't seeing results, or you see stale results, try `sudo killall wpa_supplicant` or using a different scanning method with -s.");
            thread::sleep(Duration::from_secs(5));

            if wpa_ping_success(options) {
                return Ok(());
            }
        }
    }

    Err(rerr!(
        RuwiErrorKind::FailedToStartWpaSupplicant,
        WPA_CONNECT_ERROR
    ))
}

fn wpa_ping_success<O>(options: &O) -> bool
where
    O: Global,
{
    SystemCommandRunner::new(options, "wpa_cli", &["ping"]).run_command_status_dumb()
}

pub(crate) fn kill_wpa_supplicant<O>(options: &O) -> Result<(), RuwiError>
where
    O: Global,
{
    // pkill doesn't differentiate "could not kill a process" and "no processes with that name found" except
    // in its stderr, so we instead check pgrep first and just pkill if a process was seen.
    let running_wpa_supplicant =
        SystemCommandRunner::new(options, "pgrep", &["wpa_supplicant"]).run_command_status_dumb();
    if running_wpa_supplicant {
        SystemCommandRunner::new(options, "pkill", &["wpa_supplicant"]).run_command_pass(
            RuwiErrorKind::FailedToStopWpaSupplicant,
            "Failed to stop wpa_supplicant! Are you running as root?",
        )
    } else {
        Ok(())
    }
}
