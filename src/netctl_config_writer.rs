use crate::structs::*;
use std::fs::File;
use std::io::Write;

pub fn netctl_config_write(
    options: &Options,
    network: &WirelessNetwork,
    encryption_key: &Option<String>,
) -> Result<OutputResult, OutputError> {
    let contents = get_netctl_config_contents(options, network, encryption_key)?;

    let netctl_file_name = network.essid.replace(" ", "_");
    let netctl_location = "/etc/netctl/".to_string();

    let fullpath = netctl_location + &netctl_file_name;
    let config_file = File::create(fullpath);
    config_file
        .or(Err(OutputError::CouldNotOpenConfigurationFileForWriting))?
        .write_all(contents.as_bytes())
        .or(Err(OutputError::CouldNotWriteConfigurationFile))?;
    Ok(OutputResult {
        output_type: OutputType::NetctlConfig,
    })
}

pub fn get_netctl_config_contents(
    options: &Options,
    network: &WirelessNetwork,
    encryption_key: &Option<String>,
) -> Result<String, OutputError> {
    let formatted = format!(
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
        if network.wpa {
            if let Some(ek) = encryption_key {
                format!("Key='{}'", ek)
            } else {
                "Key=''".to_string()
            }
        } else {
            "".to_string()
        },
    );

    Ok(formatted)
}
