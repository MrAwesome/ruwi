use crate::errors::*;
use crate::options::interfaces::*;
use crate::rerr;

use std::io;
use std::process::Output;
use std::process::{Command, Stdio};
#[cfg(not(test))]
use std::io::Write;

pub(super) fn spawn_and_await_output_command<O>(opts: &O, cmd: &mut Command) -> io::Result<Output>
where
    O: Global,
{
    #[cfg(test)]
    {
        dbg!(&cmd);
        let _ = opts.d();
        panic!("Prevented command usage in test!");
    }

    #[cfg(not(test))]
    {
        let spawn_res = cmd.spawn();

        if opts.d() {
            dbg!(&spawn_res);
        }

        let output_res = spawn_res?.wait_with_output();

        if opts.d() {
            dbg!(&output_res);
        }

        output_res
    }
}

pub(super) fn get_output_command<O>(
    opts: &O,
    cmd_name: &str,
    args: &[&str],
) -> Command 
    where O: Global
{
    let cmd = if opts.get_dry_run() {
        empty_command_dryrun(cmd_name, args)
    } else {
        let mut cmd = Command::new(cmd_name);
        cmd.args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        cmd
    };

    if opts.d() {
        dbg!(&cmd);
    }

    cmd
}

pub(super) fn empty_command_dryrun(cmd_name: &str, args: &[&str]) -> Command {
    eprintln!(
        "[NOTE]: Not running command in dryrun mode: `{} {}`",
        cmd_name,
        args.join(" ")
    );
    Command::new("true")
}

pub(super) fn is_cmd_installed<O>(
    opts: &O,
    cmd_name: &str,
) -> Result<(), RuwiError>
where
    O: Global,
{
    if opts.get_dry_run() {
        return Ok(());
    }

    let mut cmd = get_output_command(opts, "which", &[cmd_name]);
    let cmd_res = spawn_and_await_output_command(opts, &mut cmd);
    let is_installed = match cmd_res {
        Ok(res) => res.status.success(),
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

pub(super) fn get_prompt_command<O>(opts: &O, cmd_name: &str, args: &[&str]) -> Command
where
    O: Global,
{
    // NOTE: prompt commands are run in dryrun mode, as they should have
    //       no effect on the external state of the system, and should be
    //       tested thoroughly in integration tests.
    let mut cmd = Command::new(cmd_name);
    cmd.args(args)
        .stdin(Stdio::piped())
        // Taking stderr breaks fzf.
        //.stderr(Stdio::piped())
        .stdout(Stdio::piped());

    if opts.d() {
        dbg![&cmd];
    }

    cmd
}

pub(super) fn spawn_and_await_prompt_command<O>(
    opts: &O,
    cmd: &mut Command,
    elements: &[String],
) -> io::Result<Output>
where
    O: Global,
{
    #[cfg(test)]
    {
        dbg!(&cmd, &elements);
        let _ = opts.d();
        panic!("Prevented prompt command usage in test!");
    }

    #[cfg(not(test))]
    {
        let mut child = cmd.spawn()?;

        if opts.d() {
            dbg!(&child);
        }

        let stdin = child.stdin.as_mut().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                "Could not acquire write access to stdin.",
            )
        })?;

        stdin.write_all(elements.join("\n").as_bytes())?;

        let output = child.wait_with_output()?;

        if opts.d() {
            dbg!(&output);
        }

        Ok(output)
    }
}
