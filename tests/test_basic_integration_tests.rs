use rexpect::errors::*;
use rexpect::{spawn, spawn_bash};

mod utils;
use utils::spawn_dryrun;

extern crate ruwi;
use ruwi::interface_management::ip_interfaces::FAKE_INTERFACE_NAME;


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
    let mut p = spawn_dryrun(
        "wifi -F src/parse/samples/iw_two_different_networks.txt -s iw connect -c print -A first",
    )?;
    p.exp_string("[NOTE]: Selected network: \"Valparaiso_Guest_House 2\"")?;
    p.exp_string("Valparaiso_Guest_House 2")?;
    Ok(())
}

#[test]
fn test_iw_first_network_from_file() -> Result<()> {
    // assert_cmd would be nice for this type of stderr + stdout comparison, but it quadrupled
    // build times so we'll just use rexpect for now. The stdlib Command module would work as well.
    let mut p = spawn_dryrun(
        "wifi -F src/parse/samples/iw_two_different_networks.txt -s iw connect -c print -A first",
        
    )?;
    p.exp_string("[NOTE]: Selected network: \"Valparaiso_Guest_House 2\"")?;
    p.exp_string("Valparaiso_Guest_House 2")?;
    Ok(())
}

#[test]
fn test_print_given_essid() -> Result<()> {
    let mut p = spawn_dryrun(
        "wifi connect -e FUCKAHOL -c print",
        
    )?;
    p.exp_string("FUCKAHOL")?;
    Ok(())
}

#[test]
fn test_wired_connect() -> Result<()> {
    let mut p = spawn_dryrun(
        "wired connect",
        
    )?;
    p.exp_string(&format!("Successfully connected on \"{}\"", FAKE_INTERFACE_NAME))?;
    Ok(())
}

#[test]
fn test_iw_first_network_from_file_with_select() -> Result<()> {
    let mut p = spawn_dryrun(
        "wifi -F src/parse/samples/iw_two_different_networks.txt -s iw select -A first",
        
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
fn test_dryrun_does_not_hang() -> Result<()> {
    let mut p = spawn_dryrun(
        "wifi -s nmcli connect -c nmcli -e MADE_UP_ESSID",
        
    )?;
    p.exp_regex("Successfully connected to: \"MADE_UP_ESSID\"")?;
    let mut p = spawn_dryrun(
        "wifi -s iw connect -c netctl -e MADE_UP_ESSID",
        
    )?;
    p.exp_regex("Successfully connected to: \"MADE_UP_ESSID\"")?;
    Ok(())
}

#[test]
fn test_correct_services_started() -> Result<()> {
    // Start netctl if we want to use it to connect
    let mut p = spawn_dryrun(
        "wifi -s iw connect -c netctl -e MADE_UP_ESSID",
        
    )?;
    p.exp_regex("Not running command in dryrun mode: .?systemctl start netctl")?;

    // Don't interact with NetworkManager from an iw+netctl run
    let mut p = spawn_dryrun(
        "wifi -s iw connect -c netctl -e MADE_UP_ESSID",
        
    )?;
    assert![p.exp_string("NetworkManager").is_err()];

    // Stop networkmanager before attempting to connect with something else
    // NOTE: an nmcli scan isn't actually done here, since we're passing -e, so stopping
    //       NetworkManager doesn't make a lot of sense - but it makes testing much easier, and seems
    //       harmless, so the behavior stays for now. Plus, if we're parsing an nmcli scan from
    //       stdin or elsewhere, it's very likely it was recently run.
    let mut p = spawn_dryrun(
        "wifi -s nmcli connect -c netctl -e MADE_UP_ESSID",
        
    )?;
    p.exp_regex("Not running command in dryrun mode: .?systemctl stop NetworkManager")?;
    p.exp_regex("Not running command in dryrun mode: .?systemctl start netctl")?;

    Ok(())
}

#[test]
fn test_nmcli_cached_scan_and_synchronous_retry() -> Result<()> {
    // Scan twice in force mode, then fail before connecting
    // NOTE: we scan twice because even in force-synchronous mode,
    //       we rescan if no results at all were returned from the first scan
    let mut p = spawn_dryrun(
        "wifi -s nmcli connect -c nmcli -A known_or_fail",
        
    )?;
    let output = p.exp_eof()?;
    dbg!(&output);
    let num_count = output.matches("systemctl start NetworkManager").count();
    assert_eq![num_count, 2];
    let num_synchronous_scans = output.matches("device wifi list --rescan yes`").count();
    assert_eq![num_synchronous_scans, 1];
    let num_cached_scans = output.matches("device wifi list`").count();
    assert_eq![num_cached_scans, 1];
    Ok(())
}

#[test]
fn test_nmcli_force_synchronous_scan() -> Result<()> {
    // Scan twice in force mode, then fail before connecting
    // NOTE: we scan twice because even in force-synchronous mode,
    //       we rescan if no results at all were returned from the first scan
    let mut p = spawn_dryrun(
        "wifi -f -s nmcli connect -c nmcli -A known_or_fail",
        
    )?;
    let output = p.exp_eof()?;
    dbg!(&output);
    let num_count = output.matches("systemctl start NetworkManager").count();
    assert_eq![num_count, 2];
    let num_synchronous_scans = output.matches("device wifi list --rescan yes`").count();
    assert_eq![num_synchronous_scans, 2];
    let num_cached_scans = output.matches("device wifi list`").count();
    assert_eq![num_cached_scans, 0];
    Ok(())
}

#[test]
fn test_rescan_and_fail_in_auto_mode() -> Result<()> {
    let mut p = spawn_dryrun(
        "wifi -s nmcli connect -c netctl -A known_or_fail",
        
    )?;
    p.exp_regex("Not running command in dryrun mode: .?systemctl start NetworkManager")?;
    p.exp_regex("Not running command in dryrun mode:.*?device wifi list")?;
    p.exp_regex("Not running command in dryrun mode: .?systemctl start NetworkManager")?;
    p.exp_regex("Not running command in dryrun mode:.*?device wifi list --rescan yes")?;
    p.exp_regex("Failed to find a known network in .known_or_fail. mode")?;

    Ok(())
}

#[test]
fn test_no_nmcli_connect_with_non_nmcli_scan() -> Result<()> {
    let mut p = spawn_dryrun(
        "wifi -s iw connect -c nmcli -e MADE_UP_ESSID",
        
    )?;
    p.exp_string("Non-nmcli scan types do not work when connect_via")?;
    Ok(())
}

#[test]
fn test_clear() -> Result<()> {
    let mut p = spawn_dryrun("clear")?;
    p.exp_string("Running in dryrun mode!")?;
    let text = p.exp_eof()?;
    dbg!(&text);

    // This is a little inflexible, but since `ruwi clear` can give results in any order because it's
    // threaded, ensuring we do kill everything we expect to kill seems like a small price to pay.
    let stopped_all_netctl_profiles =
        text.contains("Not running command in dryrun mode: `netctl stop-all`");
    let killed_netctl =
        text.contains("Not running command in dryrun mode: `systemctl stop netctl`");
    let killed_nwmgr =
        text.contains("Not running command in dryrun mode: `systemctl stop NetworkManager`");
    let killed_wpa_supp =
        text.contains("Not running command in dryrun mode: `pkill wpa_supplicant`");
    dbg!(
        &stopped_all_netctl_profiles,
        &killed_netctl,
        &killed_nwmgr,
        &killed_wpa_supp
    );

    assert![stopped_all_netctl_profiles];
    assert![killed_netctl];
    assert![killed_nwmgr];
    assert![killed_wpa_supp];

    Ok(())
}
