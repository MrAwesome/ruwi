use rexpect::errors::*;
use rexpect::spawn;

#[test]
fn test_fzf_respects_ctrl_r_refresh() -> Result<()> {
    let mut p = spawn(
        "./target/debug/ruwi -D -m fzf -F src/samples/iw_many_networks.txt -c print -s iw",
        Some(2000),
    )?;
    p.exp_regex("Select a network")?;
    p.send_control('r')?;
    p.exp_string("Refresh requested")?;
    p.exp_string("Select a network")?;
    p.send_control('m')?;
    p.exp_string("Patrician Pad")?;
    p.exp_eof()?;
    Ok(())
}
