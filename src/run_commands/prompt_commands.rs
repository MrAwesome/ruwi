use super::utils::{get_prompt_command, spawn_and_await_prompt_command};

use crate::prelude::*;

pub(crate) struct PromptCommandRunner<'a, O: Global> {
    opts: &'a O,
    cmd_name: &'a str,
    args: &'a [&'a str],
    // TODO: change to &str
    elements: &'a [String],
    //#[cfg(test)]
    //expected_output: Result<String, RuwiError>,
}

impl<'a, O: Global> PromptCommandRunner<'a, O> {
    pub(crate) fn new(
        opts: &'a O,
        cmd_name: &'a str,
        args: &'a [&'a str],
        elements: &'a [String],
    ) -> Self {
        Self {
            opts,
            cmd_name,
            args,
            elements,
        }
    }

    pub(crate) fn run(&self) -> Result<String, RuwiError> {
        if self.opts.d() {
            dbg!(&self.cmd_name, &self.args, &self.elements);
        }

        let mut cmd = get_prompt_command(self.opts, self.cmd_name, self.args)?;
        let prompt_res = spawn_and_await_prompt_command(self.opts, &mut cmd, self.elements);

        if self.opts.d() {
            dbg!(&prompt_res);
        }

        match prompt_res {
            Ok(output) => {
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
            Err(err) => Err(rerr!(
                RuwiErrorKind::PromptCommandSpawnFailed,
                format!("{}", err)
            )),
        }
    }
}
