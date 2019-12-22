use crate::rerr;
use crate::structs::*;
use std::error::Error;
use std::io;
#[cfg(not(test))]
use std::io::Write;
use std::process::Output;
use std::process::{Command, Stdio};

// TODO: create unit tests which test the non-spawning parts of these functions
// TODO: assert args are actually being passed to commands


pub(crate) fn run_command_pass_stdout(
    debug: bool,
    cmd_name: &str,
    args: &[&str],
    err_kind: RuwiErrorKind,
    err_msg: &str,
) -> Result<String, RuwiError> {
    if debug {
        dbg!(&debug, &cmd_name, &args, &err_kind, &err_msg);
    }

    // TODO: allow the err_msg to be or contain stderr somehow, esp for netctl switch-to
    let output_res = run_command_output(debug, cmd_name, args);
    if let Ok(output) = &output_res {
        if output.status.success() {
            return Ok(String::from_utf8_lossy(&output.stdout).to_string());
        }
    }
    Err(rerr!(err_kind, err_msg))
}

pub(crate) fn run_command_output(
    debug: bool,
    cmd_name: &str,
    args: &[&str],
) -> Result<Output, RuwiError> {
    if debug {
        dbg!(&cmd_name, &args);
    }

    run_command_impl(debug, cmd_name, args)
        .map_err(|e| rerr!(RuwiErrorKind::FailedToRunCommand, e.description()))
}

pub(crate) fn run_command_status_dumb(
    debug: bool,
    cmd_name: &str,
    args: &[&str],
) -> bool {
    if debug {
        dbg!(&cmd_name, &args);
    }

    let res = run_command_impl(debug, cmd_name, args);

    if let Ok(output) = res {
        output.status.success()
    } else {
        false
    }

}

fn run_command_impl(debug: bool, cmd_name: &str, args: &[&str]) -> io::Result<Output> {
    let mut cmd = Command::new(cmd_name);
    cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());

    if debug {
        dbg!(&cmd);
    }

    #[cfg(test)]
    panic!("Prevented command usage in test!");

    #[cfg(not(test))]
    {
        let spawn_res = cmd.spawn();

        if debug {
            dbg!(&spawn_res);
        }

        let output_res = spawn_res?.wait_with_output();

        if debug {
            dbg!(&output_res);
        }

        output_res
    }
}

// Special runner for fzf, dmenu, etc
pub(crate) fn run_prompt_cmd(
    debug: bool,
    cmd_name: &str,
    args: &[&str],
    elements: &[String],
) -> Result<String, RuwiError> {
    if debug {
        dbg!(&cmd_name, &args, &elements);
    }

    let res = run_prompt_cmd_system_impl(debug, cmd_name, args, elements);
    if debug {
        dbg!(&res);
    }

    is_cmd_installed(debug, cmd_name)?;

    let output =
        res.map_err(|e| rerr!(RuwiErrorKind::PromptCommandSpawnFailed, format!("{}", e)))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout)
            .trim_end_matches(|x| x == '\n')
            .to_string())
    } else {
        Err(rerr!(
            RuwiErrorKind::PromptCommandFailed,
            "Prompt command exited with non-zero exit code."
        ))
    }
}

fn run_prompt_cmd_system_impl(
    debug: bool,
    cmd_name: &str,
    args: &[&str],
    elements: &[String],
) -> io::Result<Output> {
    if debug {
        dbg!(&cmd_name, &args, &elements);
    }

    let mut cmd = Command::new(cmd_name);
    let cmd = cmd
        .args(args)
        .stdin(Stdio::piped())
        // Taking stderr breaks fzf.
        //.stderr(Stdio::piped())
        .stdout(Stdio::piped());

    if debug {
        dbg![&cmd];
    }

    #[cfg(test)]
    panic!("Prevented prompt command usage in test!");

    #[cfg(not(test))]
    {
        let mut child = cmd.spawn()?;
        let stdin = child.stdin.as_mut().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                "Could not acquire write access to stdin.",
            )
        })?;

        stdin.write_all(elements.join("\n").as_bytes())?;

        let output = child.wait_with_output()?;

        Ok(output)
    }
}

fn is_cmd_installed(debug: bool, cmd_name: &str) -> Result<(), RuwiError> {
    let mut cmd = Command::new("which");
    cmd.arg(cmd_name)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .stdout(Stdio::null());

    if debug {
        dbg!(&cmd);
    }

    #[cfg(test)]
    panic!("Prevented is_cmd_installed command usage in test!");

    #[cfg(not(test))]
    {
        let status = cmd.status();

        if debug {
            dbg!(&status);
        }

        let is_installed = match status {
            Ok(exit_status) => exit_status.success(),
            Err(_) => false,
        };

        if is_installed {
            Ok(())
        } else {
            Err(rerr!(
                RuwiErrorKind::CommandNotInstalled,
                format!("`{}` is not installed or is not in $PATH.", cmd_name),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic = "Prevented command usage in test!"]
    fn test_cmd_use_in_test_panics() {
        run_command_pass_stdout(
            true,
            "echo",
            &["lawl"],
            RuwiErrorKind::TestShouldNeverBeSeen,
            "If you see this error from a test, system commands may be running in tests!",
        )
        .unwrap();
    }

    #[test]
    #[should_panic = "Prevented prompt command usage in test!"]
    fn test_prompt_cmd_use_in_test_panics() {
        run_prompt_cmd(
            true,
            "echo",
            &["loooool"],
            &["lawl".to_string()],
        )
        .unwrap();
    }

    #[test]
    #[should_panic = "Prevented is_cmd_installed command usage in test!"]
    fn test_is_cmd_installed_use_in_test_panics() {
        is_cmd_installed(true, "FUFAJKFL").unwrap();
    }
}
