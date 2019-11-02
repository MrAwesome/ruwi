use std::process::{exit, Command, Stdio};

pub(crate) fn get_default_interface(debug: bool) -> String {
    // Other methods of determining the interface can be added here
    get_wpa_cli_ifname_interface(debug)
}

fn get_wpa_cli_ifname_interface(debug: bool) -> String {
    let output_res = Command::new("wpa_cli")
        .arg("ifname")
        .stdout(Stdio::piped())
        .output();

    let output = match &output_res {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Failed to determine interface name with wpa_cli. Try providing one manually with -i. Error: {}", e);
            exit(1);
        }
    };

    let stdout = String::from(String::from_utf8_lossy(&output.stdout));

    if output.status.success() {
        let lastline = stdout.lines().last();
        if let Some(txt) = lastline {
            let ifname = String::from(txt);
            if debug {
                dbg!(&ifname);
            }
            return ifname;
        }
    }

    match output.status.code() {
        Some(num) => match num {
            127 => {
                eprintln!("`wpa_cli` is not available for determining the interface name - please install it, or manually provide an interface name with -i.");
                exit(1);
            }
            _ => {
                generic_wpa_cli_ifname_failure(&stdout);
            }
        },
        None => {
            generic_wpa_cli_ifname_failure(&stdout);
        }
    }
}

fn generic_wpa_cli_ifname_failure(stdout: &str) -> ! {
    eprintln!("Failed to determine interface name with wpa_cli. Try providing one manually with -i.\n\nOutput of `wpa_cli ifname`: {}", stdout);
    exit(1);
}
