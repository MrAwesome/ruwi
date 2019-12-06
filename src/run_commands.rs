#[cfg(not(test))]
use crate::errbox;
use crate::structs::*;
#[cfg(not(test))]
use std::io::Write;
use std::process::Output;
#[cfg(not(test))]
use std::process::{Command, Stdio};

pub(crate) fn run_command_pass_stdout(
    debug: bool,
    cmd: &str,
    args: &[&str],
    err_msg: &str,
) -> Result<String, ErrBox> {
    if debug {
        dbg!(&debug, &cmd, &args, &err_msg);
    }

    #[cfg(test)]
    return Ok("FAKE_COMMAND_OUTPUT".to_string());

    #[cfg(not(test))]
    {
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
}

pub(crate) fn run_command_output(debug: bool, cmd: &str, args: &[&str]) -> Result<Output, ErrBox> {
    if debug {
        dbg!(&cmd, &args);
    }

    #[cfg(test)]
    panic!("Prevented command usage in test!");

    #[cfg(not(test))]
    {
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
}

// Special runner for fzf, dmenu, etc
pub(crate) fn run_prompt_cmd(
    debug: bool,
    cmd_name: &str,
    args: &[&str],
    elements: Vec<String>,
) -> Result<String, ErrBox> {
    if debug {
        dbg!(&cmd_name, &args, &elements);
    }

    #[cfg(test)]
    panic!("Prevented prompt command usage in test!");

    #[cfg(not(test))]
    {
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
            .ok_or_else(|| errbox!("Stdin to prompt failed"))?;
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
}
