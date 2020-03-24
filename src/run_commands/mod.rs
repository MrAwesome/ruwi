mod prompt_commands;
mod system_commands;
mod utils;

pub(crate) use system_commands::SystemCommandRunner;
pub(crate) use prompt_commands::PromptCommandRunner;

// TODO: instead of panicking, allow a test-only "give me this output" method? is there a crate which will do that for you?
// TODO: integration test binary safety in archlinux test
// TODO: find a way to namespace O for modules like this
// TODO: time silent vs output command

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;
    use crate::options::GlobalOptions;

    fn get_default_opts(is_debug: bool, is_dry_run: bool) -> GlobalOptions {
        GlobalOptions::builder()
            .debug(is_debug)
            .dry_run(is_dry_run)
            .build()
    }

    #[test]
    #[should_panic = "Prevented command usage in test!"]
    fn test_cmd_use_in_test_panics() {
        SystemCommandRunner::new(
            &get_default_opts(true, false),
            "echo",
            &["lawl"],
        ).run_command_pass_stdout(
            RuwiErrorKind::TestShouldNeverBeSeen,
            "If you see this error from a test, system commands may be running in tests!",
        ).unwrap();
    }

    #[test]
    #[should_panic = "Prevented command usage in test!"]
    fn test_cmd_silent_use_in_test_panics() {
        SystemCommandRunner::new(
            &get_default_opts(true, false),
            "echo",
            &["lawl"],
        ).
        run_command_status_dumb();
    }

    #[test]
    #[should_panic = "Prevented command usage in test!"]
    fn test_prompt_cmd_use_in_test_panics() {
        PromptCommandRunner::new(
            &get_default_opts(true, false),
            "echo",
            &["loooool"],
            &["lawl".to_string()],
        ).run().unwrap();
    }

    #[test]
    fn test_empty_command_returns_empty() {
        let output = utils::empty_command_dryrun("echo", &["LAWL"]).output().unwrap();
        assert![output.stdout.is_empty()];
        assert![output.stderr.is_empty()];
        assert![output.status.success()];
    }
}
