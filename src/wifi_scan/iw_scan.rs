use crate::enums::*;
use crate::errors::*;
use crate::interface_management::ip_interfaces::*;
use crate::options::interfaces::*;
use crate::rerr;
use crate::run_commands::SystemCommandRunner;
use crate::structs::*;
use std::process::Output;

#[cfg(not(test))]
use std::thread;
#[cfg(not(test))]
use std::time::Duration;

use crate::wifi_scan::{
    ALLOWED_SYNCHRONOUS_RETRIES, DEVICE_OR_RESOURCE_BUSY_EXIT_CODE, SYNCHRONOUS_RETRY_DELAY_SECS,
};

// TODO: make function, include exact command being run
// TODO: fix this failure showing up in dryruns
const IW_SCAN_DUMP_ERR_MSG: &str = concat!(
    "Failed to load cached list of seen networks with `iw`. Is it installed? ",
    "You can also select a different scanning method with -s (try 'wpa_cli' or 'iwlist'), ",
    "or you can manually specify an essid with -e.",
);

const IW_SCAN_SYNC_ERR_MSG: &str = concat!(
    "Failed to scan with `iw`. Is it installed? ",
    "You can also select a different scanning method with -s (try 'wpa_cli' or 'iwlist'), ",
    "or you can manually specify an essid with -e.",
);

pub(crate) fn run_iw_scan<O>(
    options: &O,
    interface: &WifiIPInterface,
    scan_type: ScanType,
    synchronous_rescan: &Option<SynchronousRescanType>,
) -> Result<ScanResult, RuwiError>
where
    O: Global + Wifi,
{
    interface.bring_up(options)?;
    let scan_output = if options.get_force_synchronous_scan() || synchronous_rescan.is_some() {
        run_iw_scan_synchronous(options, interface)?
    } else {
        let mut scan_output = run_iw_scan_dump(options, interface)?;
        if scan_output.is_empty() {
            scan_output = run_iw_scan_synchronous(options, interface)?;
        } else {
            run_iw_scan_trigger(options, interface).ok();
        }
        scan_output
    };

    Ok(ScanResult {
        scan_type,
        scan_output,
    })
}

fn run_iw_scan_synchronous<O>(options: &O, interface: &WifiIPInterface) -> Result<String, RuwiError>
where
    O: Global + Wifi,
{
    run_iw_scan_synchronous_impl(options, interface, run_iw_scan_synchronous_cmd)
}

fn run_iw_scan_synchronous_impl<O, F>(
    options: &O,
    interface: &WifiIPInterface,
    mut synchronous_scan_func: F,
) -> Result<String, RuwiError>
where
    O: Global + Wifi,
    F: FnMut(&O, &WifiIPInterface) -> Result<Output, RuwiError>,
{
    #[cfg(not(test))]
    abort_ongoing_iw_scan(options, interface).ok();

    let mut have_given_busy_notice = false;
    let mut retries = ALLOWED_SYNCHRONOUS_RETRIES;
    loop {
        let synchronous_run_output = synchronous_scan_func(options, interface)?;

        if synchronous_run_output.status.success() {
            return Ok(String::from_utf8_lossy(&synchronous_run_output.stdout).to_string());
        } else if synchronous_run_output.status.code() == Some(DEVICE_OR_RESOURCE_BUSY_EXIT_CODE) {
            retries -= 1;
            if retries > 0 {
                if !have_given_busy_notice {
                    eprintln!("[NOTE]: Wifi interface is busy, waiting for results...");
                    have_given_busy_notice = true;
                }

                #[cfg(not(test))]
                thread::sleep(Duration::from_secs_f64(SYNCHRONOUS_RETRY_DELAY_SECS));
                #[cfg(test)]
                dbg!(SYNCHRONOUS_RETRY_DELAY_SECS);

                continue;
            } else {
                return Err(rerr!(
                    RuwiErrorKind::IWSynchronousScanRanOutOfRetries,
                    format!(
                        "Ran out of retries waiting for {} to become available for scanning with `iw`. Is NetworkManager or another service running?", 
                        interface.get_ifname()
                        ),
                ));
            }
        } else {
            return Err(rerr!(
                RuwiErrorKind::IWSynchronousScanFailed,
                IW_SCAN_SYNC_ERR_MSG
            ));
        }
    }
}

fn run_iw_scan_synchronous_cmd<O>(
    options: &O,
    interface: &WifiIPInterface,
) -> Result<Output, RuwiError>
where
    O: Global,
{
    SystemCommandRunner::new( 
        options,
        "iw",
        &[interface.get_ifname(), "scan"],
    ).run_command_output_raw(
        RuwiErrorKind::FailedToRunIWScanSynchronous,
        IW_SCAN_SYNC_ERR_MSG,
    )
}

fn run_iw_scan_dump<O>(options: &O, interface: &WifiIPInterface) -> Result<String, RuwiError>
where
    O: Global,
{
    SystemCommandRunner::new(
        options,
        "iw",
        &[interface.get_ifname(), "scan", "dump"],
    ).run_command_pass_stdout(
        RuwiErrorKind::FailedToRunIWScanDump,
        IW_SCAN_DUMP_ERR_MSG,
    )
}

fn run_iw_scan_trigger<O>(options: &O, interface: &WifiIPInterface) -> Result<String, RuwiError>
where
    O: Global,
{
    // Initiate a rescan. This command should return instantaneously.
    SystemCommandRunner::new( 
        options,
        "iw",
        &[interface.get_ifname(), "scan", "trigger"],
    ).run_command_pass_stdout(
        RuwiErrorKind::FailedToRunIWScanTrigger,
        "Triggering scan with iw failed. This should be ignored.",
    )
}

#[cfg(not(test))]
fn abort_ongoing_iw_scan<O>(options: &O,
    interface: &WifiIPInterface,
    ) -> Result<String, RuwiError>
where
    O: Global,
{
    SystemCommandRunner::new(
        options,
        "iw",
        &[interface.get_ifname(), "scan", "abort"],
    ).run_command_pass_stdout(
        RuwiErrorKind::FailedToRunIWScanAbort,
        "Aborting iw scan iw failed. This should be ignored.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::wifi::connect::WifiConnectOptions;
    use crate::wifi_scan::tests::*;

    #[test]
    fn test_synchronous_scan_pass() {
        let options = &WifiConnectOptions::default();
        let interface = &WifiIPInterface::default();
        let res = run_iw_scan_synchronous_impl(options, interface, command_pass);

        assert_eq![res.ok().unwrap().trim(), FAKE_OUTPUT];
    }

    #[test]
    fn test_synchronous_scan_failed() {
        let options = &WifiConnectOptions::default();
        let interface = &WifiIPInterface::default();
        let res = run_iw_scan_synchronous_impl(options, interface, command_fail_with_exitcode_1);

        assert_eq![
            res.err().unwrap().kind,
            RuwiErrorKind::IWSynchronousScanFailed
        ];
    }

    #[test]
    fn test_synchronous_scan_ran_out_of_retries() {
        let options = &WifiConnectOptions::default();
        let interface = &WifiIPInterface::default();
        let res = run_iw_scan_synchronous_impl(options, interface, command_fail_with_device_or_resource_busy);

        assert_eq![
            res.err().unwrap().kind,
            RuwiErrorKind::IWSynchronousScanRanOutOfRetries
        ];
    }

    #[test]
    fn test_synchronous_scan_retried_successfully() {
        let options = &WifiConnectOptions::default();
        let interface = &WifiIPInterface::default();
        let mut allowed_retries = 2;
        let res = run_iw_scan_synchronous_impl(options, interface, |opts, iface| {
            allowed_retries -= 1;
            if allowed_retries > 0 {
                command_fail_with_device_or_resource_busy(opts, iface)
            } else {
                command_pass(opts, iface)
            }
        });

        assert_eq![res.ok().unwrap().trim(), FAKE_OUTPUT];
    }

    #[test]
    fn test_synchronous_scan_ran_out_of_retries_explicit() {
        let options = &WifiConnectOptions::default();
        let interface = &WifiIPInterface::default();
        let mut allowed_retries = ALLOWED_SYNCHRONOUS_RETRIES + 1;
        let res = run_iw_scan_synchronous_impl(options, interface, |opts, iface| {
            allowed_retries -= 1;
            if allowed_retries > 0 {
                command_fail_with_device_or_resource_busy(opts, iface)
            } else {
                command_pass(opts, iface)
            }
        });

        assert_eq![
            res.err().unwrap().kind,
            RuwiErrorKind::IWSynchronousScanRanOutOfRetries
        ];
    }
}
