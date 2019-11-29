use crate::errbox;
use crate::select::*;
use crate::structs::*;
use std::collections::HashSet;

const KNOWN_TOKEN: &'static str = " (KNOWN)";

pub(crate) fn select_network(
    options: &Options,
    networks: &SortedNetworks,
) -> Result<AnnotatedWirelessNetwork, ErrBox> {
    select_network_impl(options, &networks.networks, select_network_method)
}

// TODO: clarify names
fn select_network_method(
    options: &Options,
    selection_tokens: Vec<String>,
) -> Result<String, ErrBox> {
    match &options.selection_method {
        SelectionMethod::Dmenu => {
            run_dmenu(&options, &"Select a network:".to_string(), selection_tokens)
        }
        SelectionMethod::Fzf => {
            run_fzf(&options, &"Select a network:".to_string(), selection_tokens)
        }
    }
}

fn select_network_impl<'a, F>(
    options: &'a Options,
    networks: &[AnnotatedWirelessNetwork],
    selector: F,
) -> Result<AnnotatedWirelessNetwork, ErrBox>
where
    F: FnOnce(&'a Options, Vec<String>) -> Result<String, ErrBox>,
{
    let selection_tokens = get_names_and_markers_for_selection(&networks);
    let selected_network_line = selector(options, selection_tokens)?;

    let selected_network_name = selected_network_line.trim_end_matches(KNOWN_TOKEN);

    let selected_network = networks.iter().find(|nw| nw.essid == selected_network_name);

    let res = match selected_network {
        Some(nw) => Ok(nw.clone()),
        None => Err(errbox!("No matching networks for selection")),
    };

    if options.debug {
        dbg![&res];
    }
    res
}

pub(crate) fn get_names_and_markers_for_selection(
    networks: &[AnnotatedWirelessNetwork],
) -> Vec<String> {
    let mut seen_network_names = HashSet::new();
    let mut tokens_for_selection = vec![];
    for nw in networks {
        let essid = nw.essid.clone();
        if !seen_network_names.contains(&essid) {
            seen_network_names.insert(essid.clone());

            let token = get_token_for_selection(&nw);
            tokens_for_selection.push(token);
        }
    }
    tokens_for_selection
}

fn get_token_for_selection(nw: &AnnotatedWirelessNetwork) -> String {
    match nw.known {
        true => nw.essid.clone() + KNOWN_TOKEN,
        false => nw.essid.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_network() -> Result<(), ErrBox> {
        let essid = "lol".to_string();
        let options = Options::default();
        let network = AnnotatedWirelessNetwork {
            essid,
            ..Default::default()
        };
        let networks = &[network.clone()];
        let selector = |_, _| {
            Ok(networks
                .iter()
                .next()
                .map_or("DOOOK".to_string(), |nw| nw.essid.clone()))
        };
        let nw = select_network_impl(&options, networks, selector)?;
        assert_eq![network, nw];
        Ok(())
    }

    #[test]
    fn test_get_token_for_known_network() {
        let essid = "DOOK".to_string();
        let nw = AnnotatedWirelessNetwork {
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
        let nw = AnnotatedWirelessNetwork {
            essid: essid.clone(),
            known: false,
            ..Default::default()
        };
        let token = get_token_for_selection(&nw);
        assert_eq![token, essid];
    }

    #[test]
    fn test_unique_nw_name_sort() {
        let networks = vec![
            AnnotatedWirelessNetwork {
                essid: "DOOK".to_string(),
                signal_strength: Some(-5),
                ..Default::default()
            },
            AnnotatedWirelessNetwork {
                essid: "BOYS".to_string(),
                signal_strength: Some(-47),
                ..Default::default()
            },
            AnnotatedWirelessNetwork {
                essid: "DOOK".to_string(),
                signal_strength: Some(-49),
                ..Default::default()
            },
            AnnotatedWirelessNetwork {
                essid: "YES".to_string(),
                signal_strength: Some(-89),
                ..Default::default()
            },
        ];
        let unique_network_names = get_names_and_markers_for_selection(&networks);
        let expected_names = vec!["DOOK".to_string(), "BOYS".to_string(), "YES".to_string()];

        assert_eq![unique_network_names, expected_names];
    }
}
