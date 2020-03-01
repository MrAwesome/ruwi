use crate::errors::*;
use crate::options::interfaces::Global;
use crate::rerr;
use crate::run_commands::run_command_pass_stdout;

use super::super::LinuxIPLinkDevice;

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
pub(crate) fn get_all_interfaces<O>(opts: &O) -> Result<Vec<LinuxIPLinkDevice>, RuwiError>
where
    O: Global,
{
    let output = run_command_pass_stdout(
        opts,
        "ip",
        &["-j", "link", "show"],
        RuwiErrorKind::FailedToRunIPLinkShow,
        "Failed to discover interfaces with `ip link show`! It should be included with the 'iproute2' package.",
    )?;
    Ok(serde_json::from_str(&output).unwrap())
}

pub(crate) fn get_interface_by_name<O>(opts: &O, name: &str) -> Result<LinuxIPLinkDevice, RuwiError>
where
    O: Global,
{
    let interfaces = get_all_interfaces(opts)?;
    interfaces
        .iter()
        .find(|x| x.ifname == name)
        .ok_or_else(|| {
            rerr!(
                RuwiErrorKind::NoInterfaceFoundWithGivenName,
                format!(
                    "No interface with name \"{}\" was found in: \"{}\"",
                    name,
                    interfaces
                        .iter()
                        .map(|x| x.ifname.as_ref())
                        .collect::<Vec<&str>>()
                        .join("\", \"")
                ),
            )
        })
        .map(Clone::clone)
}

// TODO: test JSON parsing here
