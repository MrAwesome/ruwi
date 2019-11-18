use std::io;
use std::process::{Command, Output, Stdio};

pub(crate) fn run_command_stdout(
    debug: bool,
    cmd: &str,
    args: &[&str],
    err_msg: &str,
) -> io::Result<String> {
    let output = run_command_output(debug, cmd, args);
    match &output {
        Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).to_string()),

        Err(_e) => Err(io::Error::new(io::ErrorKind::Other, err_msg)),
    }
}

pub(crate) fn run_command_output(debug: bool, cmd: &str, args: &[&str]) -> io::Result<Output> {
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

    let output = spawn_res?.wait_with_output();

    if debug {
        dbg!(&output);
    }

    output
}
