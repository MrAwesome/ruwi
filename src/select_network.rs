use crate::errbox;
use crate::select::*;
use crate::structs::*;
use std::collections::HashSet;

const KNOWN_TOKEN: &str = " (KNOWN)";

pub(crate) fn select_network(
    options: &Options,
    networks: &SortedNetworks,
) -> Result<AnnotatedWirelessNetwork, ErrBox> {
    select_network_impl(
        options,
        networks,
        prompt_user_for_selection,
        auto_select_network_method,
        auto_no_ask_select_network_method,
    )
}

fn prompt_user_for_selection(
    options: &Options,
    networks: &SortedNetworks,
) -> Result<AnnotatedWirelessNetwork, ErrBox> {
    let selection_tokens = get_names_and_markers_for_selection(&networks.networks);
    let selected_network_line = match &options.selection_method {
        SelectionMethod::Dmenu => {
            run_dmenu(&options, &"Select a network:".to_string(), selection_tokens)
        }
        SelectionMethod::Fzf => {
            run_fzf(&options, &"Select a network:".to_string(), selection_tokens)
        }
    }?;
    let selected_network_name = selected_network_line.trim_end_matches(KNOWN_TOKEN);
    let selected_network = networks
        .networks
        .iter()
        .find(|nw| nw.essid == selected_network_name);

    match selected_network {
        Some(nw) => Ok(nw.clone()),
        None => Err(errbox!(format!(
            "No network matching {} found.",
            selected_network_name
        ))),
    }
}

fn auto_select_network_method<'a, 'b, F>(
    options: &Options,
    networks: &SortedNetworks,
    prompt_method: F,
) -> Result<AnnotatedWirelessNetwork, ErrBox>
where
    F: FnOnce(&'a Options, &'b SortedNetworks) -> Result<AnnotatedWirelessNetwork, ErrBox>,
{
    let auto_selected = select_first_known(options, networks);
    match auto_selected {
        Ok(nw) => Ok(nw),
        Err(_) => prompt_user_for_selection(options, networks),
    }
}

fn auto_no_ask_select_network_method(
    options: &Options,
    networks: &SortedNetworks,
) -> Result<AnnotatedWirelessNetwork, ErrBox> {
    select_first_known(options, networks)
}

fn select_first_known(
    _options: &Options,
    networks: &SortedNetworks,
) -> Result<AnnotatedWirelessNetwork, ErrBox> {
    networks
        .networks
        .iter()
        .find(|nw| nw.known == true)
        .ok_or_else(|| errbox!("No known networks found!"))
        .map(|x| x.clone())
}

fn select_network_impl<'a, 'b, F, G, H>(
    options: &'a Options,
    networks: &'b SortedNetworks,
    manual_selector: F,
    auto_selector: G,
    auto_no_ask_selector: H,
) -> Result<AnnotatedWirelessNetwork, ErrBox>
where
    F: FnOnce(&'a Options, &'b SortedNetworks) -> Result<AnnotatedWirelessNetwork, ErrBox>,
    G: FnOnce(&'a Options, &'b SortedNetworks, F) -> Result<AnnotatedWirelessNetwork, ErrBox>,
    H: FnOnce(&'a Options, &'b SortedNetworks) -> Result<AnnotatedWirelessNetwork, ErrBox>,
{
    let selected_network = match &options.auto_mode {
        AutoMode::None => manual_selector(options, networks),
        AutoMode::Auto => auto_select_network_method(options, networks, manual_selector),
        AutoMode::AutoNoAsk => auto_no_ask_select_network_method(options, networks),
    };

    if options.debug {
        dbg![&selected_network];
    }
    selected_network
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
    if nw.known {
        nw.essid.clone() + KNOWN_TOKEN
    } else {
        nw.essid.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn select_first(
        _options: &Options,
        networks: &SortedNetworks,
    ) -> Result<AnnotatedWirelessNetwork, ErrBox> {
        networks
            .networks
            .iter()
            .next()
            .ok_or_else(|| errbox!("No networks found!"))
            .map(|x| x.clone())
    }

    #[test]
    fn test_select_first_network() -> Result<(), ErrBox> {
        let options = Options::default();
        let essid1 = "lol".to_string();
        let essid2 = "lol".to_string();
        let network1 = AnnotatedWirelessNetwork {
            essid: essid1,
            ..Default::default()
        };
        let network2 = AnnotatedWirelessNetwork {
            essid: essid2,
            ..Default::default()
        };
        let networks = SortedNetworks {
            networks: vec![network1.clone(), network2.clone()],
        };
        let selector = select_first;
        // TODO: MAD UNIT TESTS BRO
        let todo = "";
        let auto_selector = |_, _, _| {
            Err(errbox!(
                "Used auto selector when auto should not have been enabled."
            ))
        };
        let auto_no_ask_selector = |_, _| {
            Err(errbox!(
                "Used auto-no-ask selector when auto should not have been enabled."
            ))
        };
        let nw = select_network_impl(
            &options,
            &networks,
            selector,
            auto_selector,
            auto_no_ask_selector,
        )?;
        assert_eq![network1, nw];
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
