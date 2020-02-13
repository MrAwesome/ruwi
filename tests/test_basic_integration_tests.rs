use rexpect::errors::*;
use rexpect::{spawn, spawn_bash};

#[test]
fn test_cli_help() -> Result<()> {
    let mut p = spawn("./target/debug/ruwi --help", Some(20))?;
    p.exp_string("USAGE:")?;
    p.exp_string("FLAGS:")?;
    p.exp_string("OPTIONS:")?;
    Ok(())
}

#[test]
fn test_runtime_basic_print() -> Result<()> {
    let mut p = spawn(
        "./target/debug/ruwi -D wifi -F src/parse/samples/iw_two_different_networks.txt -s iw connect -c print -A first",
        Some(200),
    )?;
    p.exp_string("[NOTE]: Selected network: \"Valparaiso_Guest_House 2\"")?;
    p.exp_string("Valparaiso_Guest_House 2")?;
    Ok(())
}

#[test]
fn test_iw_first_network_from_file() -> Result<()> {
    // assert_cmd would be nice for this type of stderr + stdout comparison, but it quadrupled
    // build times so we'll just use rexpect for now. The stdlib Command module would work as well.
    let mut p = spawn(
        "./target/debug/ruwi -D wifi -F src/parse/samples/iw_two_different_networks.txt -s iw connect -c print -A first",
        Some(200),
    )?;
    p.exp_string("[NOTE]: Selected network: \"Valparaiso_Guest_House 2\"")?;
    p.exp_string("Valparaiso_Guest_House 2")?;
    Ok(())
}

#[test]
fn test_print_given_essid() -> Result<()> {
    let mut p = spawn(
        "./target/debug/ruwi -D wifi connect -e FUCKAHOL -c print",
        Some(200),
    )?;
    p.exp_string("FUCKAHOL")?;
    Ok(())
}

#[test]
fn test_iw_first_network_from_file_with_select() -> Result<()> {
    let mut p = spawn(
        "./target/debug/ruwi -D wifi -F src/parse/samples/iw_two_different_networks.txt -s iw select -A first",
        Some(200),
    )?;
    p.exp_string("[NOTE]: Selected network: \"Valparaiso_Guest_House 2\"")?;
    p.exp_regex("Valparaiso_Guest_House 2")?;
    Ok(())
}

#[test]
fn test_iw_many_networks_from_stdin_with_select() -> Result<()> {
    let mut p = spawn_bash(Some(200))?;
    p.execute(
        "cat src/parse/samples/iw_many_networks.txt | ./target/debug/ruwi -D wifi -s iw -I select -A first",
        ".NOTE.: Selected network: \"Patrician Pad\"",
    )?;
    p.exp_regex("Patrician Pad")?;
    Ok(())
}

#[test]
fn test_clear() -> Result<()> {
    let mut p = spawn(
        "./target/debug/ruwi -D clear",
        Some(200),
    )?;
    p.exp_string("Running in dryrun mode!")?;
    let text = p.exp_eof()?;

    // This is a little inflexible, but since `ruwi clear` can give results in any order because it's
    // threaded, ensuring we do kill everything we expect to kill seems like a small price to pay.
    let killed_netctl = text.contains("Not running command in dryrun mode: `systemctl stop netctl`");
    let killed_nwmgr = text.contains("Not running command in dryrun mode: `systemctl stop NetworkManager`");
    let killed_wpa_supp = text.contains("Not running command in dryrun mode: `pkill wpa_supplicant`");

    dbg!(text);
    dbg!(killed_netctl, killed_nwmgr, killed_wpa_supp);

    assert![killed_netctl];
    assert![killed_nwmgr];
    assert![killed_wpa_supp];

    Ok(())
}
