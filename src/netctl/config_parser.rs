use crate::common::*;

const ESSID_TOKEN: &str = "ESSID=";

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

#[cfg(test)]
mod tests {
    use super::*;

    static ETHERNET_SAMPLE: &str = include_str!("samples/ethernet_dhcp");


    #[test]
    fn test_find_strings() {

    }
}
