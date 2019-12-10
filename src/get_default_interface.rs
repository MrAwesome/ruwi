use crate::rerr;
#[cfg(not(test))]
use crate::run_commands::*;
use crate::structs::*;

pub(crate) fn get_default_interface(debug: bool) -> Result<String, RuwiError> {
    // NOTE: Other methods of determining the interface can be added here
    let interface = get_interface_with_iw(debug);

    if debug {
        dbg![&interface];
    }

    interface
}

fn get_interface_with_iw(debug: bool) -> Result<String, RuwiError> {
    #[cfg(test)]
    {
        dbg!(&debug);
        return Ok("FAKE_INTERFACE".to_string());
    }

    #[cfg(not(test))]
    {
        let err_msg = concat!(
            "Failed to determine interface name with iw. Is it installed?\n",
            "Check the output of `iw dev`, or provide an interface manually with -i.",
        );
        let iw_dev_output = run_command_pass_stdout(
            debug,
            "iw",
            &["dev"],
            RuwiErrorKind::FailedToRunIWDev,
            err_msg,
        )?;

        get_interface_from_iw_output(&iw_dev_output)
    }
}

fn get_interface_from_iw_output(iw_output: &str) -> Result<String, RuwiError> {
    let interfaces = iw_output
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

    match interfaces.first() {
        Some(intf) => Ok(intf.to_string()),
        None => Err(rerr!(
            RuwiErrorKind::NoInterfacesFoundWithIW,
            "No interfaces found with `iw dev`!"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_interface_from_iw_output() -> Result<(), RuwiError> {
        let iw_dev_output = "phy#0
Interface wlp3s0
        ifindex 3
        wdev 0x1
        addr a0:88:b4:59:53:2c
        ssid Patrician Pad
        type managed
        channel 157 (5785 MHz), width: 40 MHz, center1: 5795 MHz
        txpower 15.00 dBm";

        let interface = get_interface_from_iw_output(iw_dev_output)?;
        assert_eq!["wlp3s0", interface];
        Ok(())
    }

    #[test]
    fn test_get_interface_from_malformed_iw_output() -> Result<(), RuwiError> {
        let iw_dev_output = "jfdklsajfdklsajfkdlsjfjdkkkkkkkd";

        let res = get_interface_from_iw_output(iw_dev_output);
        assert![res.is_err()];
        Ok(())
    }
}
