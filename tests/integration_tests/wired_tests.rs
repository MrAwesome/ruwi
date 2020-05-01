use rexpect::errors::*;
use super::utils::*;

fn basic_wired_dryrun_test(connect_via: &str) -> Result<()> {
    let mut p = spawn_dryrun(
        &format!("wired connect -c {}", connect_via),
    )?;
    p.exp_regex(&format!("Not running command in dryrun mode: `{}.*DRYRUN_FAKE_INTERFACE`", connect_via))?;
    p.exp_string(&format!("Successfully connected on \"DRYRUN_FAKE_INTERFACE\" using {}!", connect_via))?;
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
    let interface_name = "SO_FAKE";
    let mut p = spawn_dryrun(
        &format!("wired -i {} connect -c netctl", interface_name),
    )?;
    p.exp_string(&format!("Would write the following config contents to \"/etc/netctl/ethernet-{}\"", interface_name))?;
    p.exp_string(&format!("Interface={}", interface_name))?;
    p.exp_string("Connection=ethernet")?;
    p.exp_string("IP=dhcp")?;
    p.exp_string(&format!("Successfully connected on \"{}\" using netctl!", interface_name))?;
    Ok(())
}

#[test]
fn test_netctl_manual_profile_name() -> Result<()> {
    let interface_name = "SO_FAKE_PART_TWO";
    let netctl_profile = "MUH_PROFILE";
    let mut p = spawn_dryrun(
        &format!("wired -i {} connect -c netctl -p {}", interface_name, netctl_profile),
    )?;
    p.exp_string(&format!("Not running command in dryrun mode: `netctl switch-to {}`", netctl_profile))?;
    p.exp_string(&format!("Successfully connected on \"{}\" using netctl!", interface_name))?;
    Ok(())
}
