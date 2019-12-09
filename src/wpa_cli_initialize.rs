use crate::structs::*;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

const WPA_CONNECT_ERROR: &str = "Error initializing wpa_supplicant to use wpa_cli. Specify another scan method with -s, or add the following to /etc/wpa_supplicant/wpa_supplicant.conf and try again:

ctrl_interface=/run/wpa_supplicant
ctrl_interface_group=wheel
update_config=1

See https://wiki.archlinux.org/index.php/WPA_supplicant#Connecting_with_wpa_cli for more info.";

pub(crate) fn initialize_wpa_cli(options: &Options) -> Result<(), ErrBox> {
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
        let supplicant_status = Command::new("wpa_supplicant")
            .arg("-B")
            .arg("-i")
            .arg("wlp3s0")
            .arg("-c")
            .arg("/etc/wpa_supplicant/wpa_supplicant.conf")
            .stdout(Stdio::null())
            .status();

        if options.debug {
            dbg!(&supplicant_status);
        }

        if let Ok(stat) = supplicant_status {
            if stat.success() {
                let scan_status = Command::new("wpa_cli")
                    .arg("scan")
                    .stdout(Stdio::null())
                    .status();

                if options.debug {
                    dbg![&scan_status];
                }

                eprintln!("[NOTE]: Sleeping to wait for results from wpa_cli. This should only happen when you first start wpa_supplicant. If you aren't seeing results, or you see stale results, try `sudo killall wpa_supplicant` or using a different scanning method with -s.");
                thread::sleep(Duration::from_secs(5));

                if wpa_ping_success(options) {
                    return Ok(());
                }
            }
        }
    }

    Err(rerr!(
        RuwiErrorKind::FailedToConnectViaWPACli,
        WPA_CONNECT_ERROR
    ))
}

fn wpa_ping_success(options: &Options) -> bool {
    let ping_status = Command::new("wpa_cli")
        .arg("ping")
        .stdout(Stdio::null())
        .status();

    if options.debug {
        dbg![&ping_status];
    }

    if let Ok(s) = ping_status {
        s.success()
    } else {
        false
    }
}
