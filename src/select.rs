use crate::errbox;
use crate::run_commands::*;
use crate::structs::*;

use std::io;
use std::io::BufRead;
use std::io::Write;

pub(crate) fn run_dmenu(
    options: &Options,
    prompt: &str,
    elements: Vec<String>,
) -> Result<String, ErrBox> {
    run_prompt_cmd(options.debug, "dmenu", &["-i", "-p", prompt], elements)
}

pub(crate) fn run_fzf(
    options: &Options,
    prompt: &str,
    elements: Vec<String>,
) -> Result<String, ErrBox> {
    run_prompt_cmd(
        options.debug,
        "fzf",
        &["--layout", "reverse", &format!("--prompt={}", prompt)],
        elements,
    )
}

pub(crate) fn run_stdin_prompt_single_line(
    _options: &Options,
    prompt: &str,
    _elements: Vec<String>,
) -> Result<String, ErrBox> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let stdin = io::stdin();
    let line = stdin
        .lock()
        .lines()
        .next()
        .expect("Failed to read line from stdin!");
    line.map_err(|e| errbox!(e))
}
