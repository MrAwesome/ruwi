use crate::errbox;
use crate::structs::*;
use std::io;
use std::io::Write;
use std::process::{Command, Output, Stdio};

// TODO: for unit tests, have a CommandArgs struct which can optionally include given_result for mocking?

// TODO: make sure all instances of this are okay with passing
pub(crate) fn run_command_pass_stdout(
    debug: bool,
    cmd: &str,
    args: &[&str],
    err_msg: &str,
) -> Result<String, ErrBox> {
    let output_res = run_command_output(debug, cmd, args);
    match &output_res {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(errbox!(err_msg))
            }
        }

        Err(_e) => Err(errbox!(err_msg)),
    }
}

pub(crate) fn run_command_output(debug: bool, cmd: &str, args: &[&str]) -> Result<Output, ErrBox> {
    if debug {
        dbg![(&cmd, &args)];
    }

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

    output_res.map_err(|e| errbox!(e))
}

// Special runner for fzf, dmenu, etc
pub(crate) fn run_prompt_cmd(
    debug: bool,
    cmd_name: &str,
    args: &[&str],
    elements: Vec<String>,
) -> Result<String, ErrBox> {
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
    let stdin = child
        .stdin
        .as_mut()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Stdin to prompt failed"))?;
    stdin.write_all(input_text.as_bytes())?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout)
            .trim_end_matches(|x| x == '\n')
            .to_string())
    } else {
        Err(errbox!("Prompt command exited with non-zero exit code"))
    }
}
