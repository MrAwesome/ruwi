use rexpect::errors::*;
use rexpect::spawn;

#[test]
fn test_fzf_ctrl_c_exits() -> Result<()> {
    let mut p = spawn(
        "./target/debug/ruwi -D -m fzf -F src/samples/iw_many_networks.txt -c print -s iw",
        Some(2000),
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
