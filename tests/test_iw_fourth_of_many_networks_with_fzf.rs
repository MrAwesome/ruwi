use rexpect::errors::*;
use rexpect::spawn;

#[test]
fn test_iw_fourth_of_many_networks_with_fzf() -> Result<()> {
    let mut p = spawn(
        "./target/debug/ruwi -D -m fzf -F src/samples/iw_many_networks.txt -c print -s iw",
        Some(2000),
    )?;
    p.exp_regex("Select a network")?;
    p.send_control('n')?;
    p.send_control('n')?;
    p.send_control('n')?;
    p.send_control('m')?;
    p.exp_string("alltheinternets")?;
    p.exp_eof()?;
    Ok(())
}
