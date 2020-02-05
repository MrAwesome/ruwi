use crate::rerr;
use crate::options::interfaces::*;
use crate::errors::*;
use std::error::Error;
use std::io;
#[cfg(not(test))]
use std::io::Write;
use std::process::Output;
use std::process::{Command, ExitStatus, Stdio};

// TODO: find a way to namespace O for modules like this

pub(crate) fn run_command_pass<O>(
    opts: &O,
    cmd_name: &str,
    args: &[&str],
    err_kind: RuwiErrorKind,
    err_msg: &str,
) -> Result<(), RuwiError>
where
    O: Global,
{
    if opts.d() {
        dbg!(&cmd_name, &args, &err_kind, &err_msg);
    }

    // TODO: allow the err_msg to be or contain stderr somehow, esp for netctl switch-to
    let output_res = run_command_silent_impl(opts, cmd_name, args);
    if let Ok(output) = &output_res {
        if output.success() {
            return Ok(());
        }
    }
    Err(rerr!(err_kind, err_msg))
}

pub(crate) fn run_command_pass_stdout<O>(
    opts: &O,
    cmd_name: &str,
    args: &[&str],
    err_kind: RuwiErrorKind,
    err_msg: &str,
) -> Result<String, RuwiError>
where
    O: Global,
{
    if opts.d() {
        dbg!(&cmd_name, &args, &err_kind, &err_msg);
    }

    // TODO: allow the err_msg to be or contain stderr somehow, esp for netctl switch-to
    let output_res = run_command_output(opts, cmd_name, args);
    if let Ok(output) = &output_res {
        if output.status.success() {
            return Ok(String::from_utf8_lossy(&output.stdout).to_string());
        }
    }
    Err(rerr!(err_kind, err_msg))
}

pub(crate) fn run_command_output<O>(
    opts: &O,
    cmd_name: &str,
    args: &[&str],
) -> Result<Output, RuwiError>
where
    O: Global,
{
    if opts.d() {
        dbg!(&cmd_name, &args);
    }

    // TODO: instead of e.description, use stderr?
    run_command_impl(opts, cmd_name, args)
        .map_err(|e| rerr!(RuwiErrorKind::FailedToRunCommand, e.description()))
}

pub(crate) fn run_command_status_dumb<O>(opts: &O, cmd_name: &str, args: &[&str]) -> bool
where
    O: Global,
{
    if opts.d() {
        dbg!(&cmd_name, &args);
    }

    let res = run_command_silent_impl(opts, cmd_name, args);

    if let Ok(output) = res {
        output.success()
    } else {
        false
    }
}

fn run_command_impl<O>(opts: &O, cmd_name: &str, args: &[&str]) -> io::Result<Output>
where
    O: Global,
{
    let mut cmd = get_output_command(opts, cmd_name, args);
    spawn_and_await_output_command(opts, &mut cmd)
}

fn get_output_command<O>(opts: &O, cmd_name: &str, args: &[&str]) -> Command
where
    O: Global,
{
    let cmd = if opts.get_dry_run() {
        empty_command_dryrun(cmd_name, args)
    } else {
        let mut cmd = Command::new(cmd_name);
        cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());
        cmd
    };

    if opts.d() {
        dbg!(&cmd);
    }

    cmd
}

fn spawn_and_await_output_command<O>(opts: &O, cmd: &mut Command) -> io::Result<Output>
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

fn run_command_silent_impl<O>(opts: &O, cmd_name: &str, args: &[&str]) -> io::Result<ExitStatus>
where
    O: Global,
{
    let mut cmd = get_silent_command(opts, cmd_name, args);
    spawn_and_await_silent_command(opts, &mut cmd)
}

fn get_silent_command<O>(opts: &O, cmd_name: &str, args: &[&str]) -> Command
where
    O: Global,
{
    let cmd = if opts.get_dry_run() {
        empty_command_dryrun(cmd_name, args)
    } else {
        let mut cmd = Command::new(cmd_name);
        cmd.args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null());
        cmd
    };

    if opts.d() {
        dbg!(&cmd);
    }

    cmd
}

fn spawn_and_await_silent_command<O>(opts: &O, cmd: &mut Command) -> io::Result<ExitStatus>
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

        let run_res = spawn_res?.wait();

        if opts.d() {
            dbg!(&run_res);
        }

        run_res
    }
}

// Special runner for fzf, dmenu, etc
pub(crate) fn run_prompt_cmd<O>(
    opts: &O,
    cmd_name: &str,
    args: &[&str],
    elements: &[String],
) -> Result<String, RuwiError>
where
    O: Global,
{
    if opts.d() {
        dbg!(&cmd_name, &args, &elements);
    }

    let res = run_prompt_cmd_system_impl(opts, cmd_name, args, elements);
    if opts.d() {
        dbg!(&res);
    }

    is_cmd_installed(opts, cmd_name)?;

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

fn run_prompt_cmd_system_impl<O>(
    opts: &O,
    cmd_name: &str,
    args: &[&str],
    elements: &[String],
) -> io::Result<Output>
where
    O: Global,
{
    let mut cmd = get_prompt_command(opts, cmd_name, args);
    spawn_and_await_prompt_command(opts, &mut cmd, elements)
}

fn get_prompt_command<O>(opts: &O, cmd_name: &str, args: &[&str]) -> Command
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

fn spawn_and_await_prompt_command<O>(
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

fn is_cmd_installed<O>(opts: &O, cmd_name: &str) -> Result<(), RuwiError>
where
    O: Global,
{
    if opts.get_dry_run() {
        return Ok(());
    }

    let status = run_command_silent_impl(opts, "which", &[cmd_name]);
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

fn empty_command_dryrun(cmd_name: &str, args: &[&str]) -> Command {
    eprintln!(
        "[NOTE]: Not running command in dryrun mode: `{} {}`",
        cmd_name,
        args.join(" ")
    );
    Command::new("true")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::GlobalOptions;

    #[test]
    #[should_panic = "Prevented command usage in test!"]
    fn test_cmd_use_in_test_panics() {
        run_command_pass_stdout(
            &GlobalOptions::builder().debug(true).dry_run(false).build(),
            "echo",
            &["lawl"],
            RuwiErrorKind::TestShouldNeverBeSeen,
            "If you see this error from a test, system commands may be running in tests!",
        )
        .unwrap();
    }

    #[test]
    #[should_panic = "Prevented command usage in test!"]
    fn test_cmd_output_use_in_test_panics() {
        run_command_output(
            &GlobalOptions::builder().debug(true).dry_run(false).build(),
            "echo",
            &["lawl"],
        )
        .unwrap();
    }

    #[test]
    #[should_panic = "Prevented command usage in test!"]
    fn test_cmd_silent_use_in_test_panics() {
        run_command_status_dumb(
            &GlobalOptions::builder().debug(true).dry_run(false).build(),
            "echo",
            &["lawl"],
        );
    }

    #[test]
    #[should_panic = "Prevented prompt command usage in test!"]
    fn test_prompt_cmd_use_in_test_panics() {
        run_prompt_cmd(
            &GlobalOptions::builder().debug(true).dry_run(false).build(),
            "echo",
            &["loooool"],
            &["lawl".to_string()],
        )
        .unwrap();
    }

    #[test]
    #[should_panic = "Prevented command usage in test!"]
    fn test_is_cmd_installed_use_in_test_panics() {
        is_cmd_installed(
            &GlobalOptions::builder().debug(true).dry_run(false).build(),
            "FUFAJKFL",
        )
        .unwrap();
    }

    #[test]
    fn test_empty_command_returns_empty() {
        let output = empty_command_dryrun("echo", &["LAWL"]).output().unwrap();
        assert![output.stdout.is_empty()];
        assert![output.stderr.is_empty()];
        assert![output.status.success()];
    }
}
