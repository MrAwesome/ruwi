use super::LinuxIPLinkInterface;

use crate::errors::*;
use crate::options::interfaces::Global;
use crate::rerr;
use crate::run_commands::run_command_pass_stdout;

use serde_json;

// TODO: make sure ip is installed by default on Ubuntu, check the package name
// TODO: since there are multiple selections now, will you want different -m's in different contexts? so complicated, but it's nice
// TODO: tell isaac about unbearably white bassline
// TODO: run `ip -j link show`
// TODO: implement selectable
// TODO: make wifi interface discovery use this (with flag for iw)
// TODO: find correct way to identify wifi vs. wired
//
//      wlan wlp  vs   enp eth

pub(super) fn get_all_interfaces<O>(opts: &O) -> Result<Vec<LinuxIPLinkInterface>, RuwiError>
where
    O: Global,
{
    let stdout = run_command_pass_stdout(
        opts,
        "ip",
        &["-j", "link", "show"],
        RuwiErrorKind::FailedToRunIPLinkShow,
        "Failed to discover interfaces with `ip link show`! It should be included with the 'iproute2' package.",
    )?;
    Ok(process_ip_link_json(&stdout)?)
}

pub(super) fn get_interface_by_name<O>(
    opts: &O,
    name: &str,
) -> Result<LinuxIPLinkInterface, RuwiError>
where
    O: Global,
{
    let err_msg = format!("No interface named \"{}\" found with `ip link show dev {}`! Is \"iproute2\" installed? Does that interface exist? Try `ip link show`.", name, name);

    let stdout = run_command_pass_stdout(
        opts,
        "ip",
        &["-j", "link", "show", name],
        RuwiErrorKind::FailedToRunIPLinkShow,
        &err_msg,
    )?;
    let results = process_ip_link_json(&stdout)?;
    let dev = results
        .first()
        .ok_or_else(|| rerr!(RuwiErrorKind::NoInterfaceFoundWithGivenName, err_msg))?;
    Ok(dev.clone())
}

fn process_ip_link_json(stdout: &str) -> Result<Vec<LinuxIPLinkInterface>, RuwiError> {
    serde_json::from_str(&stdout).or_else(|_e| {
        Err(rerr!(
            RuwiErrorKind::FailedToParseIPLinkOutput,
            format!(
                "Failed to parse this `ip -j link show` output as JSON: {}",
                stdout
            )
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::super::OperState;
    use super::*;

    #[test]
    fn test_correct_json_parsing_multiple() -> Result<(), RuwiError> {
        let stdout = r#"[{"ifindex":1,"ifname":"lo","flags":["LOOPBACK","UP","LOWER_UP"],"mtu":65536,"qdisc":"noqueue","operstate":"UNKNOWN","linkmode":"DEFAULT","group":"default","txqlen":1000,"link_type":"loopback","address":"00:00:00:00:00:00","broadcast":"00:00:00:00:00:00"},{"ifindex":2,"ifname":"enp0s25","flags":["NO-CARRIER","BROADCAST","MULTICAST","UP"],"mtu":1500,"qdisc":"fq_codel","operstate":"DOWN","linkmode":"DEFAULT","group":"default","txqlen":1000,"link_type":"ether","address":"f0:de:f1:62:d0:54","broadcast":"ff:ff:ff:ff:ff:ff"},{"ifindex":3,"ifname":"wlp3s0","flags":["NO-CARRIER","BROADCAST","MULTICAST","UP"],"mtu":1500,"qdisc":"mq","operstate":"DOWN","linkmode":"DORMANT","group":"default","txqlen":1000,"link_type":"ether","address":"ae:05:f8:e3:df:67","broadcast":"ff:ff:ff:ff:ff:ff"}]"#;
        let devices = process_ip_link_json(stdout)?;
        assert_eq![
            devices,
            vec![
                LinuxIPLinkInterface {
                    ifname: "lo".to_string(),
                    link_type: "loopback".to_string(),
                    operstate: OperState::UNKNOWN,
                    flags: vec![
                        "LOOPBACK".to_string(),
                        "UP".to_string(),
                        "LOWER_UP".to_string(),
                    ],
                },
                LinuxIPLinkInterface {
                    ifname: "enp0s25".to_string(),
                    link_type: "ether".to_string(),
                    operstate: OperState::DOWN,
                    flags: vec![
                        "NO-CARRIER".to_string(),
                        "BROADCAST".to_string(),
                        "MULTICAST".to_string(),
                        "UP".to_string(),
                    ],
                },
                LinuxIPLinkInterface {
                    ifname: "wlp3s0".to_string(),
                    link_type: "ether".to_string(),
                    operstate: OperState::DOWN,
                    flags: vec![
                        "NO-CARRIER".to_string(),
                        "BROADCAST".to_string(),
                        "MULTICAST".to_string(),
                        "UP".to_string(),
                    ],
                },
            ],
        ];
        Ok(())
    }

    #[test]
    fn test_correct_json_parsing_single() -> Result<(), RuwiError> {
        let stdout = r#"[{"ifindex":3,"ifname":"wlp3s0","flags":["NO-CARRIER","BROADCAST","MULTICAST","UP"],"mtu":1500,"qdisc":"mq","operstate":"DOWN","linkmode":"DORMANT","group":"default","txqlen":1000,"link_type":"ether","address":"ae:05:f8:e3:df:67","broadcast":"ff:ff:ff:ff:ff:ff"}]"#;
        let devices = process_ip_link_json(stdout)?;
        assert_eq![
            *devices.first().unwrap(),
            LinuxIPLinkInterface {
                ifname: "wlp3s0".to_string(),
                link_type: "ether".to_string(),
                operstate: OperState::DOWN,
                flags: vec![
                    "NO-CARRIER".to_string(),
                    "BROADCAST".to_string(),
                    "MULTICAST".to_string(),
                    "UP".to_string(),
                ],
            },
        ];

        Ok(())
    }
}
