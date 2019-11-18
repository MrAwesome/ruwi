use crate::select::*;
use crate::structs::*;
use std::collections::HashSet;
use std::io;

pub(crate) fn select_network(
    options: &Options,
    sorted_available_networks: &[WirelessNetwork],
) -> io::Result<WirelessNetwork> {
    let sorted_unique_network_names = get_ordered_unique_network_names(sorted_available_networks);
    let selected_network_name = match &options.selection_method {
        SelectionMethod::Dmenu => run_dmenu(
            options,
            &"Select a network:".to_string(),
            &sorted_unique_network_names,
        ),
        SelectionMethod::Fzf => run_fzf(
            options,
            &"Select a network:".to_string(),
            &sorted_unique_network_names,
        ),
    }?;

    let selected_network = sorted_available_networks
        .iter()
        .find(|nw| nw.essid == selected_network_name);

    let res = match selected_network {
        Some(nw) => Ok(nw.clone()),
        None => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "No matching networks for selection",
        )),
    };

    options.dbg(&res);
    res
}

pub(crate) fn get_ordered_unique_network_names(
    sorted_available_networks: &[WirelessNetwork],
) -> Vec<String> {
    let mut seen_network_names = HashSet::new();
    let mut sorted_unique_network_names = vec![];
    for nw in sorted_available_networks {
        let essid = nw.essid.clone();
        if !seen_network_names.contains(&essid) {
            seen_network_names.insert(essid.clone());
            sorted_unique_network_names.push(essid);
        }
    }
    sorted_unique_network_names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_nw_name_sort() {
        let sorted_available_networks = vec![
            WirelessNetwork {
                essid: "DOOK".to_string(),
                is_encrypted: true,
                bssid: Some("f4:28:53:fe:a5:d0".to_string()),
                signal_strength: Some(-5),
                channel_utilisation: None,
            },
            WirelessNetwork {
                essid: "BOYS".to_string(),
                is_encrypted: true,
                bssid: Some("68:72:51:68:73:da".to_string()),
                signal_strength: Some(-47),
                channel_utilisation: None,
            },
            WirelessNetwork {
                essid: "DOOK".to_string(),
                is_encrypted: true,
                bssid: Some("68:72:51:68:73:da".to_string()),
                signal_strength: Some(-49),
                channel_utilisation: None,
            },
            WirelessNetwork {
                essid: "YES".to_string(),
                is_encrypted: true,
                bssid: Some("68:72:51:68:73:da".to_string()),
                signal_strength: Some(-89),
                channel_utilisation: None,
            },
        ];
        let unique_network_names = get_ordered_unique_network_names(&sorted_available_networks);
        let expected_names = vec!["DOOK".to_string(), "BOYS".to_string(), "YES".to_string()];

        assert_eq![unique_network_names, expected_names];
    }
}
