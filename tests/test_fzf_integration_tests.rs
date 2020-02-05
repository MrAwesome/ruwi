use rexpect::errors::*;
use rexpect::spawn;

#[test]
fn test_iw_fourth_of_many_networks_with_fzf() -> Result<()> {
    let mut p = spawn(
        "./target/debug/ruwi -D -m fzf wifi -F src/parse/samples/iw_many_networks.txt -s iw connect -c print",
        Some(300),
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

#[test]
fn test_iw_fourth_of_many_networks_with_fzf_select() -> Result<()> {
    let mut p = spawn(
        "./target/debug/ruwi -D -m fzf wifi -F src/parse/samples/iw_many_networks.txt -s iw select",
        Some(300),
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


#[test]
fn test_fzf_ctrl_c_exits() -> Result<()> {
    let mut p = spawn(
        "./target/debug/ruwi -D -m fzf wifi -F src/parse/samples/iw_many_networks.txt -s iw connect -c print",
        Some(300),
    )?;
    p.exp_regex("Select a network")?;
    p.send_control('c')?;
    p.exp_eof()?;
    let x = p.process.status().unwrap();
    if let rexpect::process::wait::WaitStatus::Exited(_, code) = x {
        if code != 1 {
            panic!("Ctrl-c during fzf did not cause ruwi to exit with the correct error code!");
        }
    } else {
        panic!("Ruwi process did not terminate correctly when ctrl-c was passed in fzf mode!");
    };
    Ok(())
}

#[test]
fn test_fzf_respects_ctrl_r_refresh() -> Result<()> {
    let mut p = spawn(
        "./target/debug/ruwi -D -m fzf wifi -F src/parse/samples/iw_many_networks.txt -s iw connect -c print",
        Some(300),
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

#[test]
fn test_fzf_respects_refresh_string_refresh() -> Result<()> {
    let mut p = spawn(
        "./target/debug/ruwi -D -m fzf wifi -F src/parse/samples/iw_many_networks.txt -s iw connect -c print",
        Some(300),
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
