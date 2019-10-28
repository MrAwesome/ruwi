use crate::structs::*;

use std::io;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn run_dmenu(
    _options: &Options,
    prompt: &String,
    elements: &Vec<String>,
) -> io::Result<String> {
    let input_text = elements.join("\n");
    let mut child = Command::new("dmenu")
        .arg("-i")
        .arg("-p")
        .arg(prompt)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let stdin = child.stdin.as_mut().ok_or(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Stdin to prompt failed",
    ))?;

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
