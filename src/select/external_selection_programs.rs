use crate::options::interfaces::*;
use crate::rerr;
use crate::run_commands::*;
use crate::errors::*;
use std::error::Error;

use std::io;
use std::io::BufRead;
use std::io::Write;

pub(crate) fn run_dmenu<O>(
    options: &O,
    prompt: &str,
    elements: &[String],
) -> Result<String, RuwiError> where O: Global {
    run_prompt_cmd(options, "dmenu", &["-i", "-p", prompt], elements)
}

pub(crate) fn run_fzf<O>(
    options: &O,
    prompt: &str,
    elements: &[String],
) -> Result<String, RuwiError> where O: Global {
    run_prompt_cmd(
        options,
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

pub(crate) fn run_select_nocurses<O>(
    options: &O,
    prompt: &str,
    elements: &[String],
) -> Result<String, RuwiError> where O: Global {
    run_stdin_prompt_single_line_impl(options, prompt, elements)
        .map_err(|e| rerr!(RuwiErrorKind::SingleLinePromptFailed, e.description()))
}

pub(crate) fn run_stdin_prompt_single_line<O>(
    options: &O,
    prompt: &str,
    elements: &[String],
) -> Result<String, RuwiError> where O: Global {
    run_stdin_prompt_single_line_impl(options, prompt, elements)
        .map_err(|e| rerr!(RuwiErrorKind::SingleLinePromptFailed, e.description()))
}

fn run_stdin_prompt_single_line_impl<O>(
    _options: &O,
    prompt: &str,
    elements: &[String],
) -> io::Result<String> where O: Global {
    if ! elements.is_empty() {
        eprintln!("{}", elements.join("\n"));
    }
    eprint!("{}", prompt);
    io::stdout().flush()?;
    let stdin = io::stdin();
    let line_res = stdin.lock().lines().next();

    line_res
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Failed to read from stdin."))?
}
