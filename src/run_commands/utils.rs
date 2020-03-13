use crate::errors::*;
use crate::options::interfaces::*;
use crate::rerr;

use std::io;
use std::process::Output;
use std::process::{Command, Stdio};

#[cfg(not(test))]
use std::fs::Metadata;
#[cfg(not(test))]
use std::os::unix::fs::MetadataExt;
#[cfg(not(test))]
use std::path::Path;

#[cfg(not(test))]
use nix::unistd::getuid;
#[cfg(not(test))]
use std::io::Write;
#[cfg(not(test))]
use std::os::unix::fs::PermissionsExt;

// TODO: combine codepaths for prompt and piped, since the two paths can mask functions being unused

#[derive(Debug)]
pub(super) struct FullCommandPath {
    short_cmd_name: String,
    full_pathname: String,
}

impl FullCommandPath {
    fn new_from<O>(opts: &O, short_cmd_name: &str) -> Result<Self, RuwiError>
    where
        O: Global,
    {
        let full_pathname = Self::get_full_command_path(opts, short_cmd_name)?;
        Ok(Self {
            short_cmd_name: short_cmd_name.to_string(),
            full_pathname,
        })
    }

    fn get_full_command_path<O>(opts: &O, cmd_name: &str) -> Result<String, RuwiError>
    where
        O: Global,
    {
        let mut cmd = Command::new("/bin/which");
        cmd.arg(cmd_name)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let spawn_res = spawn_and_await_output_command(opts, &mut cmd);
        if let Ok(output) = spawn_res {
            if output.status.success() {
                let full_path_untrimmed = String::from_utf8_lossy(&output.stdout).to_string();
                return Ok(full_path_untrimmed.trim().to_string());
            }
        }
        Err(rerr!(
            RuwiErrorKind::CommandNotFound,
            format!("`{}` is not installed or is not in $PATH.", cmd_name),
        ))
    }

    fn as_str(&self) -> &str {
        &self.full_pathname
    }

    #[cfg(not(test))]
    fn as_path(&self) -> &Path {
        Path::new(self.as_str())
    }
}

pub(super) fn spawn_and_await_output_command<O>(opts: &O, cmd: &mut Command) -> io::Result<Output>
where
    O: Global,
{
    #[cfg(test)]
    {
        dbg!(&cmd);
        let _ = opts;
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
) -> Result<Command, RuwiError>
where
    O: Global,
{
    Ok(
    if opts.get_dry_run() {
        empty_command_dryrun(cmd_name, args)
    } else {
        let full_path = FullCommandPath::new_from(opts, cmd_name)?;
        verify_command_safety(opts, &full_path)?;
        make_piped_command_raw(full_path, args)
    }
    )
}

pub(super) fn get_prompt_command<O>(
    opts: &O,
    cmd_name: &str,
    args: &[&str],
) -> Result<Command, RuwiError>
where
    O: Global,
{
    let full_path = FullCommandPath::new_from(opts, cmd_name)?;
    verify_command_safety(opts, &full_path)?;
    Ok(make_prompt_stdin_command_raw(full_path, args))
}

pub(super) fn empty_command_dryrun(cmd_name: &str, args: &[&str]) -> Command {
    eprintln!(
        "[NOTE]: Not running command in dryrun mode: `{} {}`",
        cmd_name,
        args.join(" ")
    );
    Command::new("true")
}

fn make_piped_command_raw(full_path: FullCommandPath, args: &[&str]) -> Command {
    let mut cmd = Command::new(full_path.as_str());
    cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());
    cmd
}

// TODO: use VerifiedSafeFullCommandPath
fn make_prompt_stdin_command_raw(full_path: FullCommandPath, args: &[&str]) -> Command {
    let mut cmd = Command::new(full_path.as_str());
    cmd.args(args)
        .stdin(Stdio::piped())
        // Taking stderr breaks fzf.
        //.stderr(Stdio::piped())
        .stdout(Stdio::piped());
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

// TODO: unit test that this is run
pub(super) fn verify_command_safety<O>(opts: &O, cmd_path: &FullCommandPath) -> Result<(), RuwiError> 
where O: Global {
    #[cfg(test)]
    dbg!(&opts.pretend_to_be_root(), &cmd_path);
    #[cfg(not(test))]
    verify_command_safety_while_running_as_root(opts, cmd_path)?;
    Ok(())
}

#[cfg(not(test))]
fn verify_command_safety_while_running_as_root<O>(opts: &O, cmd_path: &FullCommandPath) -> Result<(), RuwiError> 
where O: Global
{
    if opts.pretend_to_be_root() || running_as_root() {
        let path_obj = cmd_path.as_path();
        let metadata_res = path_obj.metadata();
        let metadata = metadata_res.map_err(|e| {
            rerr!(
                RuwiErrorKind::UnableToReadMetadataForBinary,
                format!(
                    "Unable to read metadata for binary \"{}\". Does it exist?",
                    cmd_path.as_str()
                ),
                "Path" => match path_obj.to_str() {
                    Some(path_str) => path_str,
                    None => "Could not determine path!",
                },
                "OS Err" => e
            )
        })?;
        if is_owned_by_non_root(&metadata) || is_world_or_group_writable(&metadata) {
            return Err(rerr!(
                    RuwiErrorKind::BinaryWritableByNonRootWhenRunningAsRoot,
                    format!("Attempted to run external binary \"{}\" while running as root, but it is writable by non-root users!", cmd_path.as_str())
                    ));
        }
    }
    Ok(())
}

#[cfg(not(test))]
fn is_world_or_group_writable(metadata: &Metadata) -> bool {
    let world_writable_mask = 0o0022;

    let mode = metadata.permissions().mode();
    world_writable_mask & mode != 0
}

#[cfg(not(test))]
fn running_as_root() -> bool {
    let uid = getuid();
    uid.is_root()
}

#[cfg(not(test))]
pub(super) fn is_owned_by_non_root(metadata: &Metadata) -> bool {
    let uid = metadata.uid();
    let gid = metadata.gid();

    !(uid == 0 && gid == 0)
}
