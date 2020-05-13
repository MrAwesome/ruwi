#![allow(clippy::missing_errors_doc)]
#![allow(unused)]

use rexpect::errors::Result;
use rexpect::spawn;
use rexpect::session::PtySession;

// TODO: Find reasonable values for these.
pub(super) const DRYRUN_TIMEOUT_MS: Option<u64> = Some(400);
pub(super) const UNGUARDED_TIMEOUT_MS: Option<u64> = Some(400);

#[must_use]
#[cfg(test)]
pub fn get_dryrun_cmd_with_args(args: &str) -> String {
    format!("./target/debug/ruwi -D {}", args)
}

#[must_use]
#[cfg(test)]
pub fn get_unguarded_cmd_with_args(args: &str) -> String {
    format!("./target/debug/ruwi {}", args)
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
