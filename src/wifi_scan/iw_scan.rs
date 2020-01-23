use crate::interface_management::bring_interface_up;
use crate::rerr;
use crate::run_commands::*;
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
static IW_SCAN_DUMP_ERR_MSG: &str = concat!(
    "Failed to load cached list of seen networks with `iw`. Is it installed? ",
    "You can also select a different scanning method with -s (try 'wpa_cli' or 'iwlist'), ",
    "or you can manually specify an essid with -e.",
);

static IW_SCAN_SYNC_ERR_MSG: &str = concat!(
    "Failed to scan with `iw`. Is it installed? ",
    "You can also select a different scanning method with -s (try 'wpa_cli' or 'iwlist'), ",
    "or you can manually specify an essid with -e.",
);

pub(crate) fn run_iw_scan(options: &WifiConnectOptions, scan_type: ScanType) -> Result<ScanResult, RuwiError> {
    bring_interface_up(options)?;
    let scan_output = if options.get_force_synchronous_scan() || options.get_synchronous_retry().is_some() {
        run_iw_scan_synchronous(options)?
    } else {
        let mut scan_output = run_iw_scan_dump(options)?;
        if scan_output.is_empty() {
            scan_output = run_iw_scan_synchronous(options)?;
        } else {
            run_iw_scan_trigger(options).ok();
        }
        scan_output
    };

    Ok(ScanResult {
        scan_type,
        scan_output,
    })
}

fn run_iw_scan_synchronous(options: &WifiConnectOptions) -> Result<String, RuwiError> {
    run_iw_scan_synchronous_impl(options, run_iw_scan_synchronous_cmd)
}

fn run_iw_scan_synchronous_impl<F>(
    options: &WifiConnectOptions,
    mut synchronous_scan_func: F,
) -> Result<String, RuwiError>
where
    F: FnMut(&WifiConnectOptions) -> Result<Output, RuwiError>,
{
    #[cfg(not(test))]
    abort_ongoing_iw_scan(&options).ok();

    let mut retries = ALLOWED_SYNCHRONOUS_RETRIES;
    loop {
        let synchronous_run_output = synchronous_scan_func(options)?;

        if synchronous_run_output.status.success() {
            return Ok(String::from_utf8_lossy(&synchronous_run_output.stdout).to_string());
        } else if synchronous_run_output.status.code() == Some(DEVICE_OR_RESOURCE_BUSY_EXIT_CODE) {
            retries -= 1;
            if retries > 0 {
                #[cfg(not(test))]
                thread::sleep(Duration::from_secs_f64(SYNCHRONOUS_RETRY_DELAY_SECS));
                #[cfg(test)]
                dbg!(SYNCHRONOUS_RETRY_DELAY_SECS);
                continue;
            } else {
                return Err(rerr!(
                    RuwiErrorKind::IWSynchronousScanRanOutOfRetries,
                    IW_SCAN_SYNC_ERR_MSG
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

fn run_iw_scan_synchronous_cmd(options: &WifiConnectOptions) -> Result<Output, RuwiError> {
    run_command_output(options.d(), "iw", &[&options.get_interface(), "scan"])
}

fn run_iw_scan_dump(options: &WifiConnectOptions) -> Result<String, RuwiError> {
    run_command_pass_stdout(
        options.d(),
        "iw",
        &[&options.get_interface(), "scan", "dump"],
        RuwiErrorKind::FailedToRunIWScanDump,
        IW_SCAN_DUMP_ERR_MSG,
    )
}

fn run_iw_scan_trigger(options: &WifiConnectOptions) -> Result<String, RuwiError> {
    // Initiate a rescan. This command should return instantaneously.
    run_command_pass_stdout(
        options.d(),
        "iw",
        &[&options.get_interface(), "scan", "trigger"],
        RuwiErrorKind::FailedToRunIWScanTrigger,
        "Triggering scan with iw failed. This should be ignored.",
    )
}

#[cfg(not(test))]
fn abort_ongoing_iw_scan(options: &WifiConnectOptions) -> Result<String, RuwiError> {
    run_command_pass_stdout(
        options.d(),
        "iw",
        &[&options.get_interface(), "scan", "abort"],
        RuwiErrorKind::FailedToRunIWScanAbort,
        "Aborting iw scan iw failed. This should be ignored.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wifi_scan::tests::*;

    #[test]
    fn test_synchronous_scan_pass() {
        let options = &WifiConnectOptions::default();
        let res = run_iw_scan_synchronous_impl(options, command_pass);

        assert_eq![res.ok().unwrap().trim(), FAKE_OUTPUT];
    }

    #[test]
    fn test_synchronous_scan_failed() {
        let options = &WifiConnectOptions::default();
        let res = run_iw_scan_synchronous_impl(options, command_fail_with_exitcode_1);

        assert_eq![
            res.err().unwrap().kind,
            RuwiErrorKind::IWSynchronousScanFailed
        ];
    }

    #[test]
    fn test_synchronous_scan_ran_out_of_retries() {
        let options = &WifiConnectOptions::default();
        let res = run_iw_scan_synchronous_impl(options, command_fail_with_device_or_resource_busy);

        assert_eq![
            res.err().unwrap().kind,
            RuwiErrorKind::IWSynchronousScanRanOutOfRetries
        ];
    }

    #[test]
    fn test_synchronous_scan_retried_successfully() {
        let options = &WifiConnectOptions::default();
        let mut allowed_retries = 2;
        let res = run_iw_scan_synchronous_impl(options, |opts| {
            allowed_retries -= 1;
            if allowed_retries > 0 {
                command_fail_with_device_or_resource_busy(opts)
            } else {
                command_pass(opts)
            }
        });

        assert_eq![res.ok().unwrap().trim(), FAKE_OUTPUT];
    }

    #[test]
    fn test_synchronous_scan_ran_out_of_retries_explicit() {
        let options = &WifiConnectOptions::default();
        let mut allowed_retries = ALLOWED_SYNCHRONOUS_RETRIES + 1;
        let res = run_iw_scan_synchronous_impl(options, |opts| {
            allowed_retries -= 1;
            if allowed_retries > 0 {
                command_fail_with_device_or_resource_busy(opts)
            } else {
                command_pass(opts)
            }
        });

        assert_eq![
            res.err().unwrap().kind,
            RuwiErrorKind::IWSynchronousScanRanOutOfRetries
        ];
    }
}
