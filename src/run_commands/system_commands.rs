use super::utils::*;

use crate::errors::*;
use crate::options::interfaces::*;
use crate::rerr;
use std::error::Error;
use std::io;
use std::process::Output;

pub(crate) struct SystemCommandRunner<'a, O: Global> {
    opts: &'a O,
    cmd_name: &'a str,
    args: &'a [&'a str],
    //#[cfg(test)]
    //expected_output: Result<String, RuwiError>,
}


impl<'a, O: Global> SystemCommandRunner<'a, O> {
    pub(crate) fn new(
        opts: &'a O,
        cmd_name: &'a str,
        args: &'a [&'a str],
    ) -> Self {
        Self {
            opts,
            cmd_name,
            args
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

        let output_res = self.run_command_output_raw();
        let res = match output_res {
            Ok(output) => {
                if output.status.success() {
                    Ok(output)
                } else {
                    Err(self.format_output_and_given_err(&output, err_kind, err_msg))
                }
            }
            Err(io_err) => {
                Err(self.format_failure_to_run_command_and_given_err(&io_err, err_kind, err_msg))
            }
        };
        if res.is_err() {
            is_cmd_installed(self.opts, self.cmd_name)?;
        }
        res
    }

    pub(crate) fn run_command_output_raw(&self) -> io::Result<Output> {
        if self.opts.d() {
            dbg!(&self.cmd_name, &self.args);
        }

        // TODO: instead of e.description, use stderr?
        let mut cmd = get_output_command(self.opts, self.cmd_name, &self.args);
        spawn_and_await_output_command(self.opts, &mut cmd)
    }

    pub(crate) fn run_command_status_dumb(&self) -> bool {
        if self.opts.d() {
            dbg!(&self.cmd_name, &self.args);
        }

        let mut cmd = get_output_command(self.opts, self.cmd_name, &self.args);
        let cmd_res = spawn_and_await_output_command(self.opts, &mut cmd);

        if let Ok(output) = cmd_res {
            output.status.success()
        } else {
            false
        }
    }

    fn format_output_and_given_err(
        &self,
        output: &Output,
        err_kind: RuwiErrorKind,
        err_msg: &str,
    ) -> RuwiError {
        rerr!(
            err_kind,
            err_msg,
            "Command" => format!("`{} {}`", self.cmd_name, self.args.join(" ")),
            "STDOUT" => String::from_utf8_lossy(&output.stdout),
            "STDERR" => String::from_utf8_lossy(&output.stderr)
        )
    }

    fn format_failure_to_run_command_and_given_err(
        &self,
        io_error: &io::Error,
        err_kind: RuwiErrorKind,
        err_msg: &str,
    ) -> RuwiError {
        rerr!(
            err_kind,
            err_msg,
            "Command" => format!("`{} {}`", self.cmd_name, self.args.join(" ")),
            "OS Error" => io_error.description()
        )
    }

}
