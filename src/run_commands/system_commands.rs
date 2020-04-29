//use super::types::*;
use super::utils::*;

use crate::common::*;

use std::io;
use std::process::{Command, Output};
//use std::path::Path;

pub(crate) struct SystemCommandRunner<'a, O: Global> {
    opts: &'a O,
    cmd_name: &'a str,
    args: &'a [&'a str],
    //#[cfg(test)]
    //expected_output: Result<String, RuwiError>,
}

impl<'a, O: Global> SystemCommandRunner<'a, O> {
    pub(crate) fn new(opts: &'a O, cmd_name: &'a str, args: &'a [&'a str]) -> Self {
        Self {
            opts,
            cmd_name,
            args,
        }
    }

    pub(crate) fn run_command_pass(
        &self,
        err_kind: RuwiErrorKind,
        err_msg: &str,
    ) -> Result<(), RuwiError> {
        self.run_command_output_pass(err_kind, err_msg)?;
        Ok(())
    }

    pub(crate) fn run_command_pass_stdout(
        &self,
        err_kind: RuwiErrorKind,
        err_msg: &str,
    ) -> Result<String, RuwiError> {
        let output = self.run_command_output_pass(err_kind, err_msg)?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub(crate) fn run_command_output_pass(
        &self,
        err_kind: RuwiErrorKind,
        err_msg: &str,
    ) -> Result<Output, RuwiError> {
        if self.opts.d() {
            dbg!(&self.cmd_name, &self.args, &err_kind, &err_msg);
        }

        let mut cmd = get_output_command(self.opts, self.cmd_name, &self.args)?;
        let output_res = spawn_and_await_output_command(self.opts, &mut cmd);

        match output_res {
            Ok(output) => {
                if output.status.success() {
                    Ok(output)
                } else {
                    Err(self.format_output_and_given_err(&cmd, &output, err_kind, err_msg))
                }
            }
            Err(io_err) => Err(self
                .format_failure_to_run_command_and_given_err(&cmd, &io_err, err_kind, err_msg)),
        }
    }

    pub(crate) fn run_command_output_raw(
        &self,
        err_kind: RuwiErrorKind,
        err_msg: &str,
    ) -> Result<Output, RuwiError> {
        if self.opts.d() {
            dbg!(&self.cmd_name, &self.args, &err_kind, &err_msg);
        }

        let mut cmd = get_output_command(self.opts, self.cmd_name, &self.args)?;
        let output_res = spawn_and_await_output_command(self.opts, &mut cmd);
        output_res.map_err(|e| {
            self.format_failure_to_run_command_and_given_err(&cmd, &e, err_kind, err_msg)
        })
    }

    pub(crate) fn run_command_status_dumb(&self) -> bool {
        if self.opts.d() {
            dbg!(&self.cmd_name, &self.args);
        }

        let mut cmd = get_output_command(self.opts, self.cmd_name, &self.args).unwrap();

        let spawn_res = spawn_and_await_output_command(self.opts, &mut cmd);

        if let Ok(output) = spawn_res {
            return output.status.success();
        }

        false
    }

    pub(crate) fn check_command_exists(&self) -> bool {
        check_command_exists(self.opts, self.cmd_name)
    }

    fn format_output_and_given_err(
        &self,
        cmd: &Command,
        output: &Output,
        err_kind: RuwiErrorKind,
        err_msg: &str,
    ) -> RuwiError {
        rerr!(
            err_kind,
            err_msg,
            "Command" => format!("{:?}", cmd),
            "STDOUT" => String::from_utf8_lossy(&output.stdout),
            "STDERR" => String::from_utf8_lossy(&output.stderr)
        )
    }

    fn format_failure_to_run_command_and_given_err(
        &self,
        cmd: &Command,
        io_error: &io::Error,
        err_kind: RuwiErrorKind,
        err_msg: &str,
    ) -> RuwiError {
        rerr!(
            err_kind,
            err_msg,
            "Command" => format!("{:?}", cmd),
            "OS Error" => io_error.to_string()
        )
    }
}
