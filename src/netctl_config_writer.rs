use crate::structs::*;
use std::fs::File;
use std::io;
use std::io::Write;

pub fn netctl_config_write(
    options: &Options,
    network: &WirelessNetwork,
    encryption_key: &Option<String>,
) -> io::Result<OutputResult> {
    let contents = get_netctl_config_contents(options, network, encryption_key);

    let netctl_file_name = get_netctl_file_name(&network.essid);
    let netctl_location = "/etc/netctl/".to_string();

    let fullpath = netctl_location + &netctl_file_name;

    File::create(fullpath)?.write_all(contents.as_bytes())?;
    Ok(OutputResult {
        output_type: OutputType::NetctlConfig,
        output_output: None,
    })
}

pub(crate) fn get_netctl_file_name(essid: &str) -> String {
    essid.replace(" ", "_")
}

pub fn get_netctl_config_contents(
    options: &Options,
    network: &WirelessNetwork,
    encryption_key: &Option<String>,
) -> String {
    let wpa_line = if network.wpa {
        if let Some(ek) = encryption_key {
            format!("Key='{}'", ek)
        } else {
            unreachable!("We should have set the encryption key if wpa is set.");
        }
    } else {
        "".to_string()
    };

    format!(
        "Description='{} wifi - {}'
Interface={}
Connection=wireless
Security={}
ESSID='{}'
IP=dhcp
{}
",
        network.essid,
        if network.wpa { "wpa" } else { "open" },
        options.interface,
        if network.wpa { "wpa" } else { "none" },
        network.essid,
        wpa_line,
    )
    .trim_end_matches(|x| x == '\n')
    .to_string()
}
