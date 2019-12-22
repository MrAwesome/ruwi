use rexpect::errors::*;
use rexpect::spawn;

#[test]
fn test_fzf_respects_refresh_string_refresh() -> Result<()> {
    let mut p = spawn(
        "./target/debug/ruwi -D -m fzf -F src/samples/iw_many_networks.txt -c print -s iw",
        Some(2000),
    )?;
    p.exp_regex("Select a network")?;

    // We would rather send_line("refresh"), but fzf doesn't seem to see characters we send it
    // This locks "refresh" in as the bottom option. If others need to be below it, insert the
    // appropriate number of ctrl-p before the ctrl-m.
    {
        for _ in 0..50 {
            p.send_control('n')?;
        }
        p.send_control('m')?;
    }
    p.exp_string("Refresh requested")?;
    p.exp_string("Select a network")?;
    p.send_control('n')?;
    p.send_control('m')?;
    p.exp_string("casa")?;
    p.exp_eof()?;
    Ok(())
}
