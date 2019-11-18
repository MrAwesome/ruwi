use crate::structs::Options;
use std::io;
use std::process::{Command, Output, Stdio};

fn run_command(options: &Options, cmd: &str, args: &[&str]) -> io::Result<Output> {
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

    Ok(cmd_res?)
}
