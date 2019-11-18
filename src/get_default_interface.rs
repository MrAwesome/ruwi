use std::io;
use std::process::{Command, Stdio};

pub(crate) fn get_default_interface(debug: bool) -> io::Result<String> {
    // Other methods of determining the interface can be added here
    let interface = get_wpa_cli_ifname_interface(debug);

    if debug {
        dbg![&interface];
    }

    interface
}

fn get_wpa_cli_ifname_interface(debug: bool) -> io::Result<String> {
    let iw_dev_output = Command::new("iw")
        .arg("dev")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output();

    if debug {
        dbg![&iw_dev_output];
    }

    if let Ok(output) = iw_dev_output {
        if output.status.success() {
            // TODO: move to function, unit test
            let stdout = String::from_utf8_lossy(&output.stdout);
            let interfaces = stdout
                .lines()
                .filter(|line| line.trim().starts_with("Interface"))
                .filter_map(|line| line.split_ascii_whitespace().last())
                .collect::<Vec<&str>>();
            // TODO: provide a way to select from multiple interfaces

            if interfaces.len() > 1 {
                eprintln!(concat!(
                    "[NOTE]: Multiple interfaces detected with `iw`. Will use the first. ",
                    "Manually specify with -i if you need another interface."
                ));
            }
            if let Some(intf) = interfaces.first() {
                return Ok(intf.to_string());
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        concat!(
            "Failed to determine interface name with iw. Is it installed?\n",
            "Check the output of `iw dev`, or provide an interface manually with -i.",
        ),
    ))
}
