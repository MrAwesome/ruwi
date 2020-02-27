use crate::options::interfaces::*;
use crate::rerr;
#[cfg(not(test))]
use crate::run_commands::*;
use crate::errors::*;
use std::fmt::Debug;

#[cfg(test)]
const IW_TEST_FAKE_INTERFACE: &str = "TESTS_FAKE_INTERFACE";
const DRY_RUN_FAKE_INTERFACE: &str = "DRY_RUN_FAKE_INTERFACE";

// TODO: make interface a struct of some sort?
pub(crate) fn get_default_wifi_interface<O>(opts: &O) -> Result<String, RuwiError> 
where O: Global + Debug
{
    // TODO: push this further down the stack?
    if opts.get_dry_run() {
        return Ok(DRY_RUN_FAKE_INTERFACE.to_string());
    }
    // NOTE: Other methods of determining the interface can be added here
    // TODO: nmcli device show (look at the first two fields, find wifi (can also use for wired when that day comes)
    let interface = get_interface_with_iw(opts);

    if opts.d() {
        dbg![&interface];
    }

    interface
}

fn get_interface_with_iw<O>(opts: &O) -> Result<String, RuwiError> 
where O: Global + Debug
{
    #[cfg(test)]
    {
        dbg!(&opts);
        Ok(IW_TEST_FAKE_INTERFACE.to_string())
    }

    #[cfg(not(test))]
    {
        let err_msg = concat!(
            "Failed to determine interface name with iw. Is it installed?\n",
            "Check the output of `iw dev`, or provide an interface manually with -i.",
        );
        let iw_dev_output = run_command_pass_stdout(
            opts,
            "iw",
            &["dev"],
            RuwiErrorKind::FailedToRunIWDev,
            err_msg,
        )?;

        let interfaces = get_interfaces_from_iw_dev_output(&iw_dev_output);
        select_interface(&interfaces)
    }
}

fn get_interfaces_from_iw_dev_output(iw_dev_output: &str) -> Vec<&str> {
    iw_dev_output
        .lines()
        .filter(|line| line.trim().starts_with("Interface"))
        .filter_map(|line| line.split_ascii_whitespace().last())
        .collect::<Vec<&str>>()
}

fn select_interface(interfaces: &[&str]) -> Result<String, RuwiError> {
    // TODO: provide a way to select from multiple interfaces
    if interfaces.len() > 1 {
        eprintln!(concat!(
            "[NOTE]: Multiple interfaces detected with `iw`. Will use the first. ",
            "Manually specify with -i if you need another interface."
        ));
    }

    match interfaces.first() {
        Some(intf) => Ok((*intf).to_string()),
        None => Err(rerr!(
            RuwiErrorKind::NoInterfacesFoundWithIW,
            "No interfaces found with `iw dev`!"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::GlobalOptions;

    #[test]
    fn test_get_dryrun_interface() -> Result<(), RuwiError> {
        let opts = GlobalOptions::default();
        let interface = get_default_wifi_interface(&opts)?;
        assert_eq!(interface, DRY_RUN_FAKE_INTERFACE);
        Ok(())
    }

    #[test]
    fn test_get_interface_from_iw_dev_output() -> Result<(), RuwiError> {
        let iw_dev_output = "phy#0
Interface wlp3s0
        ifindex 3
        wdev 0x1
        addr a0:88:b4:59:53:2c
        ssid Patrician Pad
        type managed
        channel 157 (5785 MHz), width: 40 MHz, center1: 5795 MHz
        txpower 15.00 dBm";

        let interface = select_interface(&get_interfaces_from_iw_dev_output(iw_dev_output))?;
        assert_eq!["wlp3s0", interface];
        Ok(())
    }

    #[test]
    fn test_get_interface_from_malformed_iw_dev_output() -> Result<(), RuwiError> {
        let iw_dev_output = "jfdklsajfdklsajfkdlsjfjdkkkkkkkd";

        let res = select_interface(&get_interfaces_from_iw_dev_output(iw_dev_output));
        assert![res.is_err()];
        Ok(())
    }
}
