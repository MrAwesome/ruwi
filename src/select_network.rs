use crate::select::*;
use crate::structs::*;
use std::collections::HashSet;
use std::io;

const KNOWN_TOKEN: &'static str = " (KNOWN)";

pub(crate) fn select_network(
    options: &Options,
    sorted_available_networks: &[WirelessNetwork],
) -> io::Result<WirelessNetwork> {
    let sorted_unique_network_names =
        get_names_and_markers_for_selection(&sorted_available_networks);
    let selected_network_line = match &options.selection_method {
        SelectionMethod::Dmenu => run_dmenu(
            options,
            &"Select a network:".to_string(),
            sorted_unique_network_names,
        ),
        SelectionMethod::Fzf => run_fzf(
            options,
            &"Select a network:".to_string(),
            sorted_unique_network_names,
        ),
    }?;

    let selected_network_name = selected_network_line.trim_end_matches(KNOWN_TOKEN);

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

    if options.debug {
        dbg![&res];
    }
    res
}

pub(crate) fn get_names_and_markers_for_selection(
    sorted_available_networks: &[WirelessNetwork],
) -> Vec<String> {
    let mut seen_network_names = HashSet::new();
    let mut tokens_for_selection = vec![];
    for nw in sorted_available_networks {
        let essid = nw.essid.clone();
        if !seen_network_names.contains(&essid) {
            seen_network_names.insert(essid.clone());

            let token = get_token_for_selection(&nw);
            tokens_for_selection.push(token);
        }
    }
    tokens_for_selection
}

fn get_token_for_selection(nw: &WirelessNetwork) -> String {
    match nw.known {
        true => nw.essid.clone() + KNOWN_TOKEN,
        false => nw.essid.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_token_for_known_network() {
        let essid = "DOOK".to_string();
        let nw = WirelessNetwork {
            essid: essid.clone(),
            known: true,
            ..Default::default()
        };
        let token = get_token_for_selection(&nw);
        assert_eq![token, essid + KNOWN_TOKEN];
    }

    #[test]
    fn test_get_token_for_unknown_network() {
        let essid = "DOOK".to_string();
        let nw = WirelessNetwork {
            essid: essid.clone(),
            known: false,
            ..Default::default()
        };
        let token = get_token_for_selection(&nw);
        assert_eq![token, essid];
    }

    #[test]
    fn test_unique_nw_name_sort() {
        let sorted_available_networks = vec![
            WirelessNetwork {
                essid: "DOOK".to_string(),
                signal_strength: Some(-5),
                ..Default::default()
            },
            WirelessNetwork {
                essid: "BOYS".to_string(),
                signal_strength: Some(-47),
                ..Default::default()
            },
            WirelessNetwork {
                essid: "DOOK".to_string(),
                signal_strength: Some(-49),
                ..Default::default()
            },
            WirelessNetwork {
                essid: "YES".to_string(),
                signal_strength: Some(-89),
                ..Default::default()
            },
        ];
        let unique_network_names = get_names_and_markers_for_selection(&sorted_available_networks);
        let expected_names = vec!["DOOK".to_string(), "BOYS".to_string(), "YES".to_string()];

        assert_eq![unique_network_names, expected_names];
    }
}
