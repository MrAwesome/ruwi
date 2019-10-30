use crate::structs::*;

use std::io;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn run_dmenu(options: &Options, prompt: &str, elements: &[String]) -> io::Result<String> {
    let mut cmd = Command::new("dmenu");
    let cmd = cmd.arg("-i").arg("-p").arg(prompt);
    run_prompt_cmd(options, prompt, elements, cmd)
}

pub fn run_fzf(options: &Options, prompt: &str, elements: &[String]) -> io::Result<String> {
    let mut cmd = Command::new("fzf");
    let cmd = cmd.arg(&format!("--prompt={}", prompt));
    run_prompt_cmd(options, prompt, elements, cmd)
}

fn run_prompt_cmd(
    _options: &Options,
    _prompt: &str,
    elements: &[String],
    cmd: &mut Command,
) -> io::Result<String> {
    let input_text = elements.join("\n");
    let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;

    let stdin = child
        .stdin
        .as_mut()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Stdin to prompt failed"))?;

    stdin.write_all(input_text.as_bytes())?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout)
            .trim_end_matches(|x| x == '\n')
            .to_string())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Prompt command exited with non-zero exit code",
        ))
    }
}
