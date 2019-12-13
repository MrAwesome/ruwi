#[cfg(not(test))]
use crate::rerr;
use crate::structs::*;
#[cfg(not(test))]
use std::error::Error;
#[cfg(not(test))]
use std::io;
#[cfg(not(test))]
use std::io::Write;
use std::process::Output;
#[cfg(not(test))]
use std::process::{Command, Stdio};

pub(crate) fn run_command_pass_stdout(
    debug: bool,
    cmd: &str,
    args: &[&str],
    err_kind: RuwiErrorKind,
    err_msg: &str,
) -> Result<String, RuwiError> {
    if debug {
        dbg!(&debug, &cmd, &args, &err_kind, &err_msg);
    }

    #[cfg(test)]
    return Ok("FAKE_COMMAND_OUTPUT".to_string());

    #[cfg(not(test))]
    {
        // TODO: allow the err_msg to be or contain stderr somehow, esp for netctl switch-to
        let output_res = run_command_output(debug, cmd, args);
        if let Ok(output) = &output_res {
            if output.status.success() {
                return Ok(String::from_utf8_lossy(&output.stdout).to_string());
            }
        }
        Err(rerr!(err_kind, err_msg))
    }
}

pub(crate) fn run_command_output(
    debug: bool,
    cmd: &str,
    args: &[&str],
) -> Result<Output, RuwiError> {
    if debug {
        dbg!(&cmd, &args);
    }

    #[cfg(test)]
    panic!("Prevented command usage in test!");

    #[cfg(not(test))]
    run_command_impl(debug, cmd, args)
        .map_err(|e| rerr!(RuwiErrorKind::FailedToRunCommand, e.description()))
}

#[cfg(not(test))]
fn run_command_impl(debug: bool, cmd: &str, args: &[&str]) -> io::Result<Output> {
    let spawn_res = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    if debug {
        dbg!(&spawn_res);
    }

    let output_res = spawn_res?.wait_with_output();

    if debug {
        dbg!(&output_res);
    }

    output_res
}

// Special runner for fzf, dmenu, etc
pub(crate) fn run_prompt_cmd(
    debug: bool,
    cmd_name: &str,
    args: &[&str],
    elements: Vec<String>,
) -> Result<String, RuwiError> {
    if debug {
        dbg!(&cmd_name, &args, &elements);
    }

    #[cfg(test)]
    panic!("Prevented prompt command usage in test!");

    #[cfg(not(test))]
    run_prompt_cmd_impl(debug, cmd_name, args, elements)
}

#[cfg(not(test))]
fn run_prompt_cmd_impl(
    debug: bool,
    cmd_name: &str,
    args: &[&str],
    elements: Vec<String>,
) -> Result<String, RuwiError> {
    let output = run_prompt_cmd_system_impl(debug, cmd_name, args, elements);
    if debug {
        dbg!(&output);
    }

    let output =
        output.map_err(|e| rerr!(RuwiErrorKind::PromptCommandSpawnFailed, e.description()))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout)
            .trim_end_matches(|x| x == '\n')
            .to_string())
    } else {
        Err(rerr!(
            RuwiErrorKind::PromptCommandFailed,
            "Prompt command exited with non-zero exit code"
        ))
    }
}

#[cfg(not(test))]
fn run_prompt_cmd_system_impl(
    debug: bool,
    cmd_name: &str,
    args: &[&str],
    elements: Vec<String>,
) -> io::Result<Output> {
    let input_text = elements.join("\n");
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

    let mut child = cmd.spawn()?;
    let stdin = child.stdin.as_mut().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::Other,
            "Could not acquire write access to stdin.",
        )
    })?;
    stdin.write_all(input_text.as_bytes())?;

    let output = child.wait_with_output()?;

    Ok(output)
}
