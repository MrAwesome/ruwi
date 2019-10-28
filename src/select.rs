use crate::structs::*;

use std::io::Write;
use std::process::{Command, Stdio};

pub fn run_dmenu(
    options: &Options,
    prompt: &String,
    elements: &Vec<String>,
) -> Result<String, SelectionError> {
    let input_text = elements.join("\n");
    let mut child = Command::new("dmenu")
        .arg("-i")
        .arg("-p")
        .arg(prompt)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .or(Err(SelectionError::FailedToSpawnChildProcessForPrompt))?;

    let mut stdin = child
        .stdin
        .as_mut()
        .ok_or(SelectionError::FailedToOpenStdinForPrompt)?;
    stdin
        .write_all(input_text.as_bytes())
        .or(Err(SelectionError::FailedToWriteToStdinForPrompt))?;

    let output = child
        .wait_with_output()
        .or(Err(SelectionError::FailedToReadStdoutFromPrompt))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout)
            .trim_end_matches(|x| x == '\n')
            .to_string())
    } else {
        Err(SelectionError::PromptExitedWithFailure)
    }
}
