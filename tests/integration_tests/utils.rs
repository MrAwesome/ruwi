#![allow(clippy::missing_errors_doc)]
#![allow(unused)]

use rexpect::errors::Result;
use rexpect::session::PtySession;
use rexpect::session::PtyBashSession;
use rexpect::spawn;
use rexpect::spawn_bash;

// TODO: Find reasonable values for these.
pub(super) const DRYRUN_TIMEOUT_MS: Option<u64> = Some(400);
pub(super) const UNGUARDED_TIMEOUT_MS: Option<u64> = Some(400);

pub const BINARY_NAME: &str = env!("CARGO_BIN_EXE_ruwi");

#[must_use]
#[cfg(test)]
pub fn get_dryrun_cmd_with_args(args: &str) -> String {
    format!("{} -D {}", BINARY_NAME, args)
}

#[must_use]
#[cfg(test)]
pub fn get_unguarded_cmd_with_args(args: &str) -> String {
    format!("{} {}", BINARY_NAME, args)
}

#[cfg(test)]
pub fn spawn_dryrun_noargs() -> Result<PtySession> {
    let dryrun_cmd = format!("{} -D", BINARY_NAME);
    spawn(&dryrun_cmd, DRYRUN_TIMEOUT_MS)
}

#[cfg(test)]
pub fn spawn_dryrun(args: &str) -> Result<PtySession> {
    let dryrun_cmd = get_dryrun_cmd_with_args(args);
    spawn(&dryrun_cmd, DRYRUN_TIMEOUT_MS)
}

#[cfg(test)]
pub fn spawn_unguarded(args: &str) -> Result<PtySession> {
    let unguarded_cmd = get_unguarded_cmd_with_args(args);
    spawn(&unguarded_cmd, UNGUARDED_TIMEOUT_MS)
}

fn with_env_prefix_str(command: String, env_vars: &[(&str, &str)]) -> String {
    let prefix = env_vars
        .iter()
        .map(|(k, v)| format!("{}={} ", k, v))
        .collect::<Vec<String>>()
        .concat();
    [prefix, command].concat()
}

#[must_use]
#[cfg(test)]
pub fn get_dryrun_cmd_with_args_with_env(args: &str, env_vars: &[(&str, &str)]) -> String {
    let command = format!("{} -D {}", BINARY_NAME, args);
    with_env_prefix_str(command, env_vars)
}

#[must_use]
#[cfg(test)]
pub fn get_unguarded_cmd_with_args_with_env(args: &str, env_vars: &[(&str, &str)]) -> String {
    let command = format!("{} {}", BINARY_NAME, args);
    with_env_prefix_str(command, env_vars)
}

#[cfg(test)]
pub fn spawn_dryrun_noargs_with_env(env_vars: &[(&str, &str)]) -> Result<PtySession> {
    let command = format!("{} -D", BINARY_NAME);
    let dryrun_cmd = with_env_prefix_str(command, env_vars);
    spawn(&dryrun_cmd, DRYRUN_TIMEOUT_MS)
}

#[cfg(test)]
pub fn spawn_dryrun_with_env(args: &str, env_vars: &[(&str, &str)]) -> Result<PtySession> {
    let dryrun_cmd = get_dryrun_cmd_with_args_with_env(args, env_vars);
    spawn(&dryrun_cmd, DRYRUN_TIMEOUT_MS)
}

#[cfg(test)]
pub fn spawn_unguarded_with_env(args: &str, env_vars: &[(&str, &str)]) -> Result<PtySession> {
    let unguarded_cmd = get_unguarded_cmd_with_args_with_env(args, env_vars);
    spawn(&unguarded_cmd, UNGUARDED_TIMEOUT_MS)
}
