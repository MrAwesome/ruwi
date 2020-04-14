use crate::common::*;
use super::*;

const ESSID_TOKEN: &str = "ESSID=";

impl<'a> TryFrom<&NetctlRawConfig<'a>> for NetctlRawParsedFields {
    type Error = RuwiError;

    fn try_from(f: &NetctlRawConfig) -> Result<Self, RuwiError> {
        // TODO: here, or in a helper function, grab the fields you need from a netctl config blob
        unimplemented!()
    }
}

// TODO: make this more generic, get_field_from_netctl_config
fn get_field_from_netctl_config_text(contents: &str, token: &str) -> Option<String> {
    contents.lines().find_map(|line| {
        if line.starts_with(token) {
            let value = line
                .trim_start_matches(token)
                .trim_start_matches('\'')
                .trim_start_matches('"')
                .trim_end_matches('\'')
                .trim_end_matches('"')
                .to_string();
            Some(value)
        } else {
            None
        }
    })
}

// .filter_map(|entry| get_essid_from_netctl_config_file(entry).ok())
// .filter_map(|essid_entry| {
//     if let Some((essid, identifier)) = essid_entry {
//         let escaped_essid = unescape(&essid).unwrap_or(essid);
//         Some((escaped_essid, NetworkServiceIdentifier::Netctl(identifier)))
//     } else {
//         None
//     }
// })

