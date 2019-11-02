use std::io;
use std::process::{Command, Stdio};

pub(crate) fn get_default_interface(debug: bool) -> io::Result<String> {
    // Other methods of determining the interface can be added here
    get_wpa_cli_ifname_interface(debug)
}

fn get_wpa_cli_ifname_interface(debug: bool) -> io::Result<String> {
    let output_res = Command::new("wpa_cli")
        .arg("ifname")
        .stdout(Stdio::piped())
        .output();

    if debug {
        dbg!(&output_res);
    }

    if let Ok(output) = output_res {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some(lastline) = stdout.lines().last() {
                return Ok(lastline.to_string());
            }
        }
    }

    get_generic_wpa_cli_ifname_failure()
}

fn get_generic_wpa_cli_ifname_failure() -> io::Result<String> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        concat!("Failed to determine interface name with wpa_cli. Is it installed?\n",
        "Check the output of `wpa_cli scan_results`, or try providing an interface manually with -i.",
    ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    // TODO: figure out how to unit test commands appropriately
    #[test]
    fn test_wpa_cli_ifname() {
        assert_eq![
            get_wpa_cli_ifname_interface(false)
                .err()
                .unwrap()
                .description(),
            get_generic_wpa_cli_ifname_failure()
                .err()
                .unwrap()
                .description(),
        ];
    }
}
