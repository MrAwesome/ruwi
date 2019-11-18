use crate::structs::Options;
use std::io;
use std::process::{Command, Stdio};

pub(crate) fn run_command(
    options: &Options,
    cmd: &str,
    args: &[&str],
    err_msg: &str,
) -> io::Result<String> {
    options.dbg((&cmd, &args));

    let spawn_res = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    if options.debug {
        dbg!(&spawn_res);
    }

    let cmd_res = spawn_res?.wait_with_output();

    if options.debug {
        dbg!(&cmd_res);
    }

    match &cmd_res {
        Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).to_string()),

        Err(_e) => Err(io::Error::new(io::ErrorKind::Other, err_msg)),
    }
}
