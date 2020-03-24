use crate::errors::*;
use crate::interface_management::ip_interfaces::*;
use crate::options::interfaces::*;
use crate::rerr;
use crate::structs::*;
use std::fs::File;
use std::io;
use std::io::Write;

// TODO: Check if existing config with ESSID (for wifi) or interface (for raw interface connect)
// already exists, and if so just use that and don't write to it (unless a particular flag exists?)
// should annotated networks include the config name/location? sorta makes sense. we already scan
// for this with both netctl and nmcli so it's a waste to re-search for them
//
//wifi: 
// fn write_new(essid, 

pub(crate) trait NetctlConfigHandler<O>
where
    O: Global,
{
    fn get_opts(&self) -> &O;
    fn get_netctl_config_contents(&self) -> String;
    fn get_netctl_file_name(&self) -> String;

    fn get_netctl_file_location(&self) -> String {
        "/etc/netctl/".to_string()
    }

    fn write(&self) -> Result<ConfigResult, RuwiError> {
        let config_contents = self.get_netctl_config_contents();
        let netctl_file_name = self.get_netctl_file_name();
        let netctl_location = self.get_netctl_file_location();

        let fullpath = netctl_location + &netctl_file_name;

        if !self.get_opts().get_dry_run() {
            write_to_netctl_config(&fullpath, &config_contents)
                .map_err(|e| rerr!(RuwiErrorKind::FailedToWriteNetctlConfig, e.to_string()))?;
            eprintln!("[NOTE]: Wrote netctl config: {}", &fullpath);
            eprintln!(
                "[NOTE]: If you encounter problems with your connection, try 
                editing that file directly and/or running `netctl switch-to {}` as root.",
                netctl_file_name
            );
        }

        Ok(ConfigResult {
            //connection_type: WifiConnectionType::Netctl,
            config_data: ConfigData {
                config_path: Some(fullpath),
            },
        })
    }
}

pub(crate) struct WifiNetctlConfigHandler<'a, O>
where
    O: Global,
{
    opts: &'a O,
    interface: &'a WifiIPInterface,
    network: &'a AnnotatedWirelessNetwork,
    encryption_key: &'a Option<String>,
}

impl<'a, O> WifiNetctlConfigHandler<'a, O>
where
    O: Global,
{
    pub(crate) fn new(
        opts: &'a O,
        interface: &'a WifiIPInterface,
        network: &'a AnnotatedWirelessNetwork,
        encryption_key: &'a Option<String>,
    ) -> Self {
        Self {
            opts,
            interface,
            network,
            encryption_key,
        }
    }
}

impl<'a, O> NetctlConfigHandler<O> for WifiNetctlConfigHandler<'a, O>
where
    O: Global,
{
    fn get_opts(&self) -> &O {
        self.opts
    }

    fn get_netctl_config_contents(&self) -> String {
        get_wifi_netctl_config_contents(
            self.opts,
            self.interface,
            self.network,
            self.encryption_key,
        )
    }

    fn get_netctl_file_name(&self) -> String {
        // TODO: look for existing configs here?
        self.network.essid.replace(" ", "_")
    }
}

struct RawInterfaceNetctlConfigWriter<'a, O>
where
    O: Global,
{
    opts: &'a O,
    interface: &'a WiredIPInterface,
}

fn write_to_netctl_config(fullpath: &str, contents: &str) -> io::Result<()> {
    File::create(&fullpath)?.write_all(contents.as_bytes())
}

// NOTE: This is wifi-specific, but the file overall can easily be refactored to work
//       for other connection types
pub(crate) fn get_wifi_netctl_config_contents<O>(
    _options: &O,
    interface: &WifiIPInterface,
    network: &AnnotatedWirelessNetwork,
    encryption_key: &Option<String>,
) -> String
where
    O: Global,
{
    let wpa_line = if network.is_encrypted {
        format!(
            "Key='{}'",
            // TODO: see if encryption status/key can be bundled together
            encryption_key
                .as_ref()
                .expect("We should have set the encryption key if wpa is set.")
        )
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
        if network.is_encrypted { "wpa" } else { "open" },
        interface.get_ifname(),
        if network.is_encrypted { "wpa" } else { "none" },
        network.essid,
        wpa_line,
    )
    .trim_end_matches(|x| x == '\n')
    .to_string()
}
