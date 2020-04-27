use super::utils::*;
use super::structs::*;
use super::*;
use crate::common::*;
use crate::interface_management::ip_interfaces::*;

use std::fmt;

// TODO: Check if existing config with ESSID (for wifi) or interface (for raw interface connect)
// already exists, and if so just use that and don't write to it (unless a particular flag exists?)
// should annotated networks include the config name/location? sorta makes sense. we already scan
// for this with both netctl and nmcli so it's a waste to re-search for them
//
//wifi:
// fn write_new(essid,
//
//
// TODO: selector for VAR=val (Connection=wireless/ethernet, ESSID, Interface)

// TODO: split into reader, writer, and connector

impl<'a, O: Global> NetctlConfigHandler<'a, O> {
    pub(super) fn write_config_to_file<C>(&self, config: &C) -> Result<(), RuwiError>
    where
        C: NetctlConfig<'a>,
    {
        let config_text = format!("{}", config);

        let netctl_file_name = config.get_identifier().as_ref();
        let netctl_location = &self.get_netctl_cfg_dir();
        let fullpath = format!("{}{}", netctl_location, netctl_file_name);

        if self.opts.get_dry_run() {
            eprintln!(
                "Would write the following config contents to \"{}\":\n{}",
                fullpath, config_text
            );
        } else {
            write_to_netctl_config(&fullpath, &config_text)
                .map_err(|e| rerr!(RuwiErrorKind::FailedToWriteNetctlConfig, e.to_string()))?;
            eprintln!("[NOTE]: Wrote netctl config: {}", &fullpath);
            eprintln!(
                "[NOTE]: If you encounter problems with your connection, try 
                editing that file directly and/or running `netctl switch-to {}` as root.",
                netctl_file_name
            );
        }

        Ok(())
    }
}

// TODO: unit test
impl WifiNetctlConfig {
    pub(super) fn new(
        interface: &WifiIPInterface,
        network: &AnnotatedWirelessNetwork,
        encryption_key: &Option<String>,
    ) -> Self {
        let identifier = NetctlIdentifier::from(network);
        let interface_name = interface.get_ifname().to_string();
        let essid = network.get_public_name().to_string();
        let encryption_key = encryption_key.clone();
        Self::builder()
            .identifier(identifier)
            .interface_name(interface_name)
            .essid(essid)
            .encryption_key(encryption_key)
            .build()
    }

    fn as_config_text(&self) -> String {
        let is_encrypted = self.encryption_key.is_some();
        let wpa_line = if let Some(encryption_key) = &self.encryption_key {
            format!(
                "Key='{}'",
                // TODO: see if encryption status/key can be bundled together
                encryption_key
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
            self.essid,
            if is_encrypted { "wpa" } else { "open" },
            self.interface_name,
            if is_encrypted { "wpa" } else { "none" },
            self.essid,
            wpa_line,
        )
        .trim_end_matches(|x| x == '\n')
        .to_string()
    }
}

impl fmt::Display for WifiNetctlConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.as_config_text())
    }
}

impl WiredNetctlConfig {
    pub(super) fn new(interface: &WiredIPInterface, network: &AnnotatedWiredNetwork) -> Self {
        let identifier = match network.get_service_identifier() {
            Some(NetworkServiceIdentifier::Netctl(ident)) => ident.clone(),
            _ => format!("ethernet-{}", interface.get_ifname()),
        };
        let interface_name = interface.get_ifname().to_string();
        Self::builder()
            .identifier(identifier)
            .interface_name(interface_name)
            .build()
    }

    fn as_config_text(&self) -> String {
        format!(
            "Description='{} wired - {}'
Interface={}
Connection=ethernet
IP=dhcp
",
            self.identifier.as_ref(),
            self.interface_name,
            self.interface_name,
        )
        .trim_end_matches(|x| x == '\n')
        .to_string()
    }
}

impl fmt::Display for WiredNetctlConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.as_config_text())
    }
}
