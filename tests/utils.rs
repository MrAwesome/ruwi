use rexpect::errors::*;
use rexpect::spawn;
use rexpect::session::PtySession;

#[cfg(test)]
pub fn spawn_dryrun(args: &str) -> Result<PtySession> {
    let cmd = format!("./target/debug/ruwi -D {}", args);
    spawn(&cmd, Some(200))
}
