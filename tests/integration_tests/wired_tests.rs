use rexpect::errors::*;
use super::utils::*;

fn basic_wired_dryrun_test(connect_via: &str) -> Result<()> {
    let mut p = spawn_dryrun(
        &format!("wired connect -c {}", connect_via),
    )?;
    p.exp_regex(&format!("Not running command in dryrun mode: `{}.*FAKE_INTERFACE`", connect_via))?;
    p.exp_string(&format!("Successfully connected on \"FAKE_INTERFACE\" using {}!", connect_via))?;
    Ok(())
}

#[test]
fn test_dhcpcd() -> Result<()> {
    basic_wired_dryrun_test("dhcpcd")
}

#[test]
fn test_dhclient() -> Result<()> {
    basic_wired_dryrun_test("dhclient")
}

#[test]
fn test_nmcli() -> Result<()> {
    basic_wired_dryrun_test("nmcli")
}

#[test]
fn test_netctl() -> Result<()> {
    basic_wired_dryrun_test("netctl")
}

#[test]
fn test_netctl_manual_profile_name() -> Result<()> {
    let netctl_profile = "MUH_PROFILE";
    let mut p = spawn_dryrun(
        &format!("wired connect -c netctl -p {}", netctl_profile),
    )?;
    let x = p.exp_regex("Not running command in dryrun mode: `netctl.*FAKE_INTERFACE`")?;
    dbg!(x);
    let x = p.exp_string(&format!("Using manually-specified netctl profile \"{}\"", netctl_profile))?;
    dbg!(x);
    let x = p.exp_string("Successfully connected on \"FAKE_INTERFACE\" using netctl!")?;
    dbg!(x);
    Ok(())
}
