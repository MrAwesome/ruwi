use rexpect::errors::*;
use rexpect::spawn;

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
        "./target/debug/ruwi -D -F src/samples/iw_two_different_networks.txt -c print -s iw -A first",
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
        "./target/debug/ruwi -D -F src/samples/iw_two_different_networks.txt -c print -s iw -A first",
        Some(200),
    )?;
    p.exp_string("[NOTE]: Selected network: \"Valparaiso_Guest_House 2\"")?;
    p.exp_string("Valparaiso_Guest_House 2")?;
    Ok(())
}

#[test]
fn test_print_given_essid() -> Result<()> {
    let mut p = spawn(
        "./target/debug/ruwi -D -e FUCKAHOL -c print",
        Some(200),
    )?;
    p.exp_string("FUCKAHOL")?;
    Ok(())
}
