// For typedbuilder:
#![allow(clippy::used_underscore_binding)]

use crate::prelude::*;

use crate::interface_management::ip_interfaces::{LinuxIPInterface, WiredIPInterface};
use std::fmt::Debug;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScanResult {
    pub scan_type: ScanType,
    pub scan_output: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParseResult {
    pub scan_type: ScanType,
    pub seen_networks: Vec<WirelessNetwork>,
    pub line_parse_errors: Vec<(String, IndividualParseError)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndividualParseError {
    FailedToParseSignalLevel,
    FailedToUnescapeSSIDField,
    MissingIWCapabilityField,
    MissingIWSSIDField,
    MissingNmcliSeparator,
    MissingWpaCliResultField,
    ZeroLengthIWChunk,
}

// TODO: make private, provide interface?
#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub struct WirelessNetwork {
    essid: String,
    #[builder(default = false)]
    is_encrypted: bool,
    #[builder(default = None)]
    bssid: Option<String>,
    #[builder(default = None)]
    signal_strength: Option<i32>,
    #[builder(default = None)]
    channel_utilisation: Option<String>,
}

impl RuwiNetwork for WirelessNetwork {}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub struct AnnotatedWirelessNetwork {
    // Scanned fields
    essid: String,
    #[builder(default = false)]
    is_encrypted: bool,
    #[builder(default = None)]
    bssid: Option<String>,
    #[builder(default = None)]
    signal_strength: Option<i32>,
    #[builder(default = None)]
    channel_utilisation: Option<String>,

    // Non-scan annotated fields
    #[builder(default = None)]
    service_identifier: Option<NetworkingServiceIdentifier>,
}

impl Annotated for AnnotatedWirelessNetwork {
    type Orig = WirelessNetwork;

    fn from_nw(
        nw: WirelessNetwork,
        service_identifier: Option<&NetworkingServiceIdentifier>,
    ) -> Self {
        let essid = nw.essid;
        let is_encrypted = nw.is_encrypted;
        let bssid = nw.bssid;
        let signal_strength = nw.signal_strength;
        let channel_utilisation = nw.channel_utilisation;
        let service_identifier = service_identifier.map(Clone::clone);
        Self {
            essid,
            is_encrypted,
            bssid,
            signal_strength,
            channel_utilisation,
            service_identifier,
        }
    }
}

impl AnnotatedWirelessNetwork {
    pub fn is_encrypted(&self) -> bool {
        self.is_encrypted
    }
    pub fn _get_bssid(&self) -> Option<&String> {
        self.bssid.as_ref()
    }
    pub fn get_signal_strength(&self) -> Option<i32> {
        self.signal_strength
    }
    pub fn _get_channel_utilisation(&self) -> Option<&String> {
        self.channel_utilisation.as_ref()
    }

    #[cfg(test)]
    pub fn from_essid_only(essid: &str) -> Self {
        Self::builder().essid(essid).build()
    }

    #[cfg(test)]
    pub(crate) fn set_service_identifier_for_tests(
        &mut self,
        service_identifier: Option<NetworkingServiceIdentifier>,
    ) {
        self.service_identifier = service_identifier
    }
}

// TODO: most uses of this should probably be "get_essid()" instead
impl Identifiable for WirelessNetwork {
    fn get_public_name(&self) -> &str {
        &self.essid
    }
}

impl Identifiable for AnnotatedWirelessNetwork {
    fn get_public_name(&self) -> &str {
        &self.essid
    }
}

impl Known for AnnotatedWirelessNetwork {
    type ServiceIdentifier = NetworkingServiceIdentifier;

    fn is_known(&self) -> bool {
        self.service_identifier.is_some()
    }
    fn get_service_identifier(&self) -> Option<&NetworkingServiceIdentifier> {
        self.service_identifier.as_ref()
    }
}

impl RuwiNetwork for AnnotatedWirelessNetwork {}
impl AnnotatedRuwiNetwork for AnnotatedWirelessNetwork {}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub struct AnnotatedWiredNetwork {
    interface: WiredIPInterface,

    #[builder(default = None)]
    service_identifier: Option<NetworkingServiceIdentifier>,
}

impl AnnotatedWiredNetwork {
    pub fn get_ifname(&self) -> &str {
        self.interface.get_ifname()
    }
}

impl Known for AnnotatedWiredNetwork {
    type ServiceIdentifier = NetworkingServiceIdentifier;

    fn is_known(&self) -> bool {
        self.service_identifier.is_some()
    }
    fn get_service_identifier(&self) -> Option<&NetworkingServiceIdentifier> {
        self.service_identifier.as_ref()
    }
}

impl Identifiable for AnnotatedWiredNetwork {
    fn get_public_name(&self) -> &str {
        let ifname = self.interface.get_ifname();
        if let Some(ident) = self.get_service_identifier() {
            match ident {
                NetworkingServiceIdentifier::Netctl(netident) => {
                    // TODO: just the identifier? mention netctl?
                    netident.as_ref()
                }
                NetworkingServiceIdentifier::NetworkManager => ifname,
            }
        } else {
            ifname
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigResult {
    //pub connection_type: WifiConnectionType,
    pub config_data: ConfigData,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConfigData {
    pub identifier: Option<NetworkingServiceIdentifier>,
    pub config_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionResult {
    pub connection_type: WifiConnectionType,
    //pub cmd_output: Option<String>,
    //ipv4_addr: Option<String>,
    //ipv6_addr: Option<String>,
}
