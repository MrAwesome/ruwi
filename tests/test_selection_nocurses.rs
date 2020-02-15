use rexpect::errors::*;
use rexpect::spawn_bash;

// TODO: if running in opt mode, test opt here? there should be a simple way to find the binary name.
fn prompt_string_nocurses<'a>() -> &'a str {
    "./target/debug/ruwi -D -m nocurses wifi -F src/parse/samples/iw_many_networks.txt -s iw select"
}

#[test]
fn test_nocurses_with_num() -> Result<()> {
    let mut p = spawn_bash(Some(20))?;
    p.execute(prompt_string_nocurses(), "Select a network")?;
    p.send_line("4")?;
    p.exp_string("ATT3kzR3CV")?;
    p.wait_for_prompt()?;
    Ok(())
}

#[test]
fn test_nocurses_with_num_double_digits() -> Result<()> {
    let mut p = spawn_bash(Some(20))?;
    p.execute(prompt_string_nocurses(), "Select a network")?;
    p.send_line("14")?;
    p.exp_string("TEST NETWORK PLEASE IGNORE")?;
    p.wait_for_prompt()?;
    Ok(())
}

#[test]
fn test_nocurses_just_press_enter() -> Result<()> {
    let mut p = spawn_bash(Some(20))?;
    p.execute(prompt_string_nocurses(), "Select a network")?;
    p.send_line(" ")?;
    p.exp_string("Patrician Pad")?;
    p.wait_for_prompt()?;
    Ok(())
}

#[test]
fn test_nocurses_refresh() -> Result<()> {
    let mut p = spawn_bash(Some(20))?;
    p.execute(prompt_string_nocurses(), "Select a network")?;
    p.send_line("refresh")?;
    p.exp_string("Refresh requested")?;
    p.send_line(".")?;
    p.exp_string("Refresh requested")?;
    p.send_line("7")?;
    p.exp_string("GandN")?;
    p.wait_for_prompt()?;
    Ok(())
}

// TODO: implement prefix matching?
#[test]
#[ignore]
fn test_nocurses_prefix_matching() -> Result<()> {
    let mut p = spawn_bash(Some(20))?;
    p.execute(prompt_string_nocurses(), "Select a network")?;
    p.send_line("Pa")?;
    p.exp_string("Patrician Pad")?;
    p.wait_for_prompt()?;
    Ok(())
}

