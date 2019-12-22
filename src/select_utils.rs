use crate::rerr;
use crate::run_commands::*;
use crate::structs::*;
use std::error::Error;

use std::io;
use std::io::BufRead;
use std::io::Write;

pub(crate) fn run_dmenu(
    options: &Options,
    prompt: &str,
    elements: &[String],
) -> Result<String, RuwiError> {
    run_prompt_cmd(options.debug, "dmenu", &["-i", "-p", prompt], elements)
}

#[cfg_attr(test, allow(unused))]
pub(crate) fn run_fzf(
    options: &Options,
    prompt: &str,
    elements: &[String],
) -> Result<String, RuwiError> {
    run_prompt_cmd(
        options.debug,
        "fzf",
        &[
            "--layout",
            "reverse",
            &format!("--prompt={}", prompt),
            "--bind",
            "ctrl-r:execute(echo refresh)+end-of-line+unix-line-discard+print-query",
        ],
        elements,
    )
}

pub(crate) fn run_stdin_prompt_single_line(
    options: &Options,
    prompt: &str,
    elements: &[String],
) -> Result<String, RuwiError> {
    run_stdin_prompt_single_line_impl(options, prompt, elements)
        .map_err(|e| rerr!(RuwiErrorKind::SingleLinePromptFailed, e.description()))
}

fn run_stdin_prompt_single_line_impl(
    _options: &Options,
    prompt: &str,
    _elements: &[String],
) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let stdin = io::stdin();
    let line_res = stdin.lock().lines().next();

    line_res
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Failed to read from stdin."))?
}
