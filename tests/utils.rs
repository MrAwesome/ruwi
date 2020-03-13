#![allow(clippy::missing_errors_doc)]
#![allow(unused)]

use rexpect::errors::*;
use rexpect::spawn;
use rexpect::session::PtySession;


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
    let cmd = get_dryrun_cmd_with_args(args);
    spawn(&cmd, Some(200))
}
