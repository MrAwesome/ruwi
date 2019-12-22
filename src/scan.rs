use crate::interface_management::bring_interface_up;
use crate::rerr;
use crate::run_commands::*;
use crate::structs::*;
use crate::wpa_cli_initialize::initialize_wpa_cli;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::process::Output;

#[cfg(not(test))]
use std::thread;
#[cfg(not(test))]
use std::time::Duration;

static ALLOWED_SYNCHRONOUS_RETRIES: u64 = 15;
static SYNCHRONOUS_RETRY_DELAY_SECS: f64 = 0.3;

// TODO: make function, include exact command being run
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

static DEVICE_OR_RESOURCE_BUSY_EXIT_CODE: i32 = 240;

pub(crate) fn wifi_scan(options: Options) -> Result<ScanResult, RuwiError> {
    let sm = options.scan_method.clone();
    let st = options.scan_type.clone();
    let res = match sm {
        ScanMethod::ByRunning => match &st {
            ScanType::WpaCli => run_wpa_cli_scan(&options, st),
            ScanType::IW => run_iw_scan(&options, st),
            ScanType::RuwiJSON => 
                Err(rerr!(
                    RuwiErrorKind::InvalidScanTypeAndMethod,
                    "There is currently no binary for providing JSON results, you must format them yourself and pass in via stdin or from a file.",
                ))
        },
        ScanMethod::FromFile(filename) => get_scan_contents_from_file(&options, st, &filename),
        ScanMethod::FromStdin => get_scan_contents_from_stdin(&options, st),
    };

    if options.debug {
        dbg![&res];
    }
    res
}

fn get_scan_contents_from_stdin(
    _options: &Options,
    scan_type: ScanType,
) -> Result<ScanResult, RuwiError> {
    let mut stdin_contents = "".into();
    io::stdin().read_to_end(&mut stdin_contents).map_err(|e| {
        rerr!(
            RuwiErrorKind::FailedToReadScanResultsFromStdin,
            e.description()
        )
    })?;

    let scan_output = String::from_utf8_lossy(&stdin_contents).into();

    Ok(ScanResult {
        scan_type,
        scan_output,
    })
}

fn get_scan_contents_from_file(
    _options: &Options,
    scan_type: ScanType,
    filename: &str,
) -> Result<ScanResult, RuwiError> {
    let file_read_err = |e: io::Error| {
        rerr!(
            RuwiErrorKind::FailedToReadScanResultsFromFile,
            e.description()
        )
    };
    let mut file_contents = "".into();
    File::open(filename)
        .map_err(file_read_err)?
        .read_to_end(&mut file_contents)
        .map_err(file_read_err)?;

    let scan_output = String::from_utf8_lossy(&file_contents).into();

    Ok(ScanResult {
        scan_type,
        scan_output,
    })
}

fn run_wpa_cli_scan(options: &Options, scan_type: ScanType) -> Result<ScanResult, RuwiError> {
    initialize_wpa_cli(options)?;

    let err_msg = concat!(
        "Failed to scan with `wpa_cli scan_results`. ",
        "Is wpa_supplicant running? Is it installed? ",
        "You can also select a different scanning method with -s (try 'iw' or 'iwlist'), ",
        "or you can manually specify an essid with -e.",
    );

    // TODO: add scan_results latest
    let scan_output = run_command_pass_stdout(
        options.debug,
        "wpa_cli",
        &["scan_results"],
        RuwiErrorKind::FailedToScanWithWPACli,
        err_msg,
    )?;

    if options.debug {
        dbg![&scan_output];
    }
    Ok(ScanResult {
        scan_type,
        scan_output,
    })
}

// TODO: unit test
fn run_iw_scan(options: &Options, scan_type: ScanType) -> Result<ScanResult, RuwiError> {
    bring_interface_up(options)?;
    let scan_output = if options.force_synchronous_scan || options.synchronous_retry.is_some() {
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

fn run_iw_scan_synchronous(options: &Options) -> Result<String, RuwiError> {
    run_iw_scan_synchronous_impl(options, run_iw_scan_synchronous_cmd)
}

fn run_iw_scan_synchronous_impl<F>(
    options: &Options,
    mut synchronous_scan_func: F,
) -> Result<String, RuwiError>
where
    F: FnMut(&Options) -> Result<Output, RuwiError>,
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

fn run_iw_scan_synchronous_cmd(options: &Options) -> Result<Output, RuwiError> {
    run_command_output(options.debug, "iw", &[&options.interface, "scan"])
}

fn run_iw_scan_dump(options: &Options) -> Result<String, RuwiError> {
    run_command_pass_stdout(
        options.debug,
        "iw",
        &[&options.interface, "scan", "dump"],
        RuwiErrorKind::FailedToRunIWScanDump,
        IW_SCAN_DUMP_ERR_MSG,
    )
}

fn run_iw_scan_trigger(options: &Options) -> Result<String, RuwiError> {
    // Initiate a rescan. This command should return instantaneously.
    run_command_pass_stdout(
        options.debug,
        "iw",
        &[&options.interface, "scan", "trigger"],
        RuwiErrorKind::FailedToRunIWScanTrigger,
        "Triggering scan with iw failed. This should be ignored.",
    )
}

#[cfg(not(test))]
fn abort_ongoing_iw_scan(options: &Options) -> Result<String, RuwiError> {
    run_command_pass_stdout(
        options.debug,
        "iw",
        &[&options.interface, "scan", "abort"],
        RuwiErrorKind::FailedToRunIWScanAbort,
        "Aborting iw scan iw failed. This should be ignored.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::{Command, Stdio};

    static FAKE_OUTPUT: &str = "LOLWUTFAKEIWLOL";
    fn command_fail_with_exitcode(code: i32) -> Output {
        Command::new("/bin/sh")
            .arg("-c")
            .arg(format!("exit {}", code))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap()
    }

    fn command_fail_with_exitcode_1(_options: &Options) -> Result<Output, RuwiError> {
        Ok(command_fail_with_exitcode(1))
    }

    fn command_fail_with_device_or_resource_busy(_options: &Options) -> Result<Output, RuwiError> {
        Ok(command_fail_with_exitcode(
            DEVICE_OR_RESOURCE_BUSY_EXIT_CODE,
        ))
    }

    fn command_pass(_opts: &Options) -> Result<Output, RuwiError> {
        Ok(Command::new("/bin/echo")
            .arg(FAKE_OUTPUT)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap())
    }

    #[test]
    fn test_synchronous_scan_pass() {
        let options = &Options::default();
        let res = run_iw_scan_synchronous_impl(options, command_pass);

        assert_eq![res.ok().unwrap().trim(), FAKE_OUTPUT];
    }

    #[test]
    fn test_synchronous_scan_failed() {
        let options = &Options::default();
        let res = run_iw_scan_synchronous_impl(options, command_fail_with_exitcode_1);

        assert_eq![
            res.err().unwrap().kind,
            RuwiErrorKind::IWSynchronousScanFailed
        ];
    }

    #[test]
    fn test_synchronous_scan_ran_out_of_retries() {
        let options = &Options::default();
        let res = run_iw_scan_synchronous_impl(options, command_fail_with_device_or_resource_busy);

        assert_eq![
            res.err().unwrap().kind,
            RuwiErrorKind::IWSynchronousScanRanOutOfRetries
        ];
    }

    #[test]
    fn test_synchronous_scan_retried_successfully() {
        let options = &Options::default();
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
        let options = &Options::default();
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

    #[test]
    fn test_enough_time_to_retry() {
        let expected_min_secs_needed_to_abort_scan = 4.0;
        assert![
            ALLOWED_SYNCHRONOUS_RETRIES as f64 * SYNCHRONOUS_RETRY_DELAY_SECS
                > expected_min_secs_needed_to_abort_scan
        ];
    }
}
