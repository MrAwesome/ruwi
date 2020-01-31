use crate::options::interfaces::*;
use crate::run_commands::*;
use crate::errors::*;
use std::thread;
use std::time::Duration;

static WPA_CONNECT_ERROR: &str = "Error initializing wpa_supplicant to use wpa_cli. Specify another scan method with -s, or add the following to /etc/wpa_supplicant/wpa_supplicant.conf and try again:

ctrl_interface=/run/wpa_supplicant
ctrl_interface_group=wheel
update_config=1

See https://wiki.archlinux.org/index.php/WPA_supplicant#Connecting_with_wpa_cli for more info.";

pub(crate) fn initialize_wpa_supplicant<O>(options: &O) -> Result<(), RuwiError>
where
    O: Global,
{
    //        /etc/wpa_supplicant/wpa_supplicant.conf
    //    ctrl_interface=/run/wpa_supplicant
    //    ctrl_interface_group=wheel
    //    update_config=1

    // The simplest way to ensure
    // https://wiki.archlinux.org/index.php/WPA_supplicant#Connecting_with_wpa_cli
    if wpa_ping_success(options) {
        return Ok(());
    } else {
        eprintln!(
            "[NOTE]: wpa_cli was not functioning correctly. Attempting to start it manually."
        );
        let supplicant_status = run_command_status_dumb(
            options,
            "wpa_supplicant",
            &[
                "-B",
                "-i",
                "wlp3s0",
                "-c",
                "/etc/wpa_supplicant/wpa_supplicant.conf",
            ],
        );

        if supplicant_status {
            run_command_status_dumb(options, "wpa_cli", &["scan"]);

            eprintln!("[NOTE]: Sleeping to wait for results from wpa_cli. This should only happen when you first start wpa_supplicant. If you aren't seeing results, or you see stale results, try `sudo killall wpa_supplicant` or using a different scanning method with -s.");
            thread::sleep(Duration::from_secs(5));

            if wpa_ping_success(options) {
                return Ok(());
            }
        }
    }

    Err(rerr!(
        RuwiErrorKind::FailedToConnectViaWPACli,
        WPA_CONNECT_ERROR
    ))
}

fn wpa_ping_success<O>(options: &O) -> bool
where
    O: Global,
{
    let ping_status = run_command_status_dumb(options, "wpa_cli", &["ping"]);

    if options.d() {
        dbg![&ping_status];
    }

    ping_status
}

