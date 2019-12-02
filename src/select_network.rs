use crate::errbox;
use crate::select::*;
use crate::structs::*;
use std::collections::HashSet;

const KNOWN_TOKEN: &str = " (KNOWN)";
const NO_KNOWN_NETWORKS_FOUND: &str = "No known networks found!";

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

fn auto_select_network_method(
    options: &Options,
    networks: &SortedNetworks,
) -> Result<AnnotatedWirelessNetwork, ErrBox> {
    select_first_known(options, networks)
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
        .ok_or_else(|| errbox!(NO_KNOWN_NETWORKS_FOUND))
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
    G: FnOnce(&'a Options, &'b SortedNetworks) -> Result<AnnotatedWirelessNetwork, ErrBox>,
    H: FnOnce(&'a Options, &'b SortedNetworks) -> Result<AnnotatedWirelessNetwork, ErrBox>,
{
    let selected_network = match &options.auto_mode {
        AutoMode::None => manual_selector(options, networks),
        AutoMode::Auto => {
            auto_selector(options, networks).or_else(|_| manual_selector(options, networks))
        }
        AutoMode::AutoNoAsk => auto_no_ask_selector(options, networks),
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

    const FIRST_NW_NAME: &str = "FIRSTNWLOL";
    const SECND_NW_NAME: &str = "SECNDNWWUT";
    const THIRD_NW_NAME: &str = "THIRDNWOKK";

    const USED_MANUAL_WHEN_NOT_EXPECTED: &str =
        "Used manual selector when it should not have been enabled.";
    const USED_AUTO_WHEN_NOT_EXPECTED: &str =
        "Used auto selector when it should not have been enabled.";
    const USED_AUTO_NO_ASK_WHEN_NOT_EXPECTED: &str =
        "Used auto-no-ask selector when it should not have been enabled.";

    fn get_3_networks() -> SortedNetworks {
        let networks = vec![FIRST_NW_NAME, SECND_NW_NAME, THIRD_NW_NAME]
            .iter()
            .map(|name| AnnotatedWirelessNetwork {
                essid: name.to_string(),
                ..Default::default()
            })
            .collect();
        SortedNetworks { networks }
    }

    fn get_3_unknown_networks() -> SortedNetworks {
        get_3_networks()
    }

    fn get_3_networks_first_known() -> SortedNetworks {
        let mut networks = get_3_networks();
        networks.networks[0].known = true;
        networks
    }

    fn get_3_networks_last_known() -> SortedNetworks {
        let mut networks = get_3_networks();
        networks.networks[2].known = true;
        networks
    }

    fn err_should_not_have_used_manual(
        _opt: &Options,
        _nw: &SortedNetworks,
    ) -> Result<AnnotatedWirelessNetwork, ErrBox> {
        Err(errbox!(USED_MANUAL_WHEN_NOT_EXPECTED))
    }

    fn err_should_not_have_used_auto(
        _opt: &Options,
        _nw: &SortedNetworks,
    ) -> Result<AnnotatedWirelessNetwork, ErrBox> {
        Err(errbox!(USED_AUTO_WHEN_NOT_EXPECTED))
    }

    fn err_should_not_have_used_auto_no_ask(
        _opt: &Options,
        _nw: &SortedNetworks,
    ) -> Result<AnnotatedWirelessNetwork, ErrBox> {
        Err(errbox!(USED_AUTO_NO_ASK_WHEN_NOT_EXPECTED))
    }

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

    fn select_last(
        _options: &Options,
        networks: &SortedNetworks,
    ) -> Result<AnnotatedWirelessNetwork, ErrBox> {
        networks
            .networks
            .iter()
            .last()
            .ok_or_else(|| errbox!("No networks found!"))
            .map(|x| x.clone())
    }

    fn fail_to_select(
        _options: &Options,
        _networks: &SortedNetworks,
    ) -> Result<AnnotatedWirelessNetwork, ErrBox> {
        Err(errbox!("No networks found!"))
    }

    #[test]
    fn test_manually_select_first_network() -> Result<(), ErrBox> {
        let options = Options::default();
        assert_eq![options.auto_mode, AutoMode::None];
        let networks = get_3_unknown_networks();
        let manual_selector = select_first;
        let auto_selector = err_should_not_have_used_auto;
        let auto_no_ask_selector = err_should_not_have_used_auto_no_ask;
        let nw = select_network_impl(
            &options,
            &networks,
            manual_selector,
            auto_selector,
            auto_no_ask_selector,
        )?;
        assert_eq![networks.networks[0], nw];
        Ok(())
    }

    #[test]
    fn test_manually_select_last_network() -> Result<(), ErrBox> {
        let options = Options::default();
        assert_eq![options.auto_mode, AutoMode::None];
        let networks = get_3_unknown_networks();
        let manual_selector = select_last;
        let auto_selector = err_should_not_have_used_auto;
        let auto_no_ask_selector = err_should_not_have_used_auto_no_ask;
        let nw = select_network_impl(
            &options,
            &networks,
            manual_selector,
            auto_selector,
            auto_no_ask_selector,
        )?;
        assert_eq![networks.networks[2], nw];
        Ok(())
    }

    #[test]
    fn test_fail_to_manually_select() -> Result<(), ErrBox> {
        let options = Options::default();
        assert_eq![options.auto_mode, AutoMode::None];
        let networks = get_3_unknown_networks();
        let manual_selector = select_first_known;
        let auto_selector = err_should_not_have_used_auto;
        let auto_no_ask_selector = err_should_not_have_used_auto_no_ask;
        let nw = select_network_impl(
            &options,
            &networks,
            manual_selector,
            auto_selector,
            auto_no_ask_selector,
        );
        assert![nw.is_err()];
        if let Err(err) = nw {
            assert_eq![err.description(), NO_KNOWN_NETWORKS_FOUND];
        }
        Ok(())
    }

    #[test]
    fn test_auto_no_ask_first_known() -> Result<(), ErrBox> {
        let mut options = Options::default();
        options.auto_mode = AutoMode::AutoNoAsk;

        let networks = get_3_networks_first_known();
        let manual_selector = err_should_not_have_used_manual;
        let auto_selector = err_should_not_have_used_auto;
        let auto_no_ask_selector = select_first_known;
        let nw = select_network_impl(
            &options,
            &networks,
            manual_selector,
            auto_selector,
            auto_no_ask_selector,
        )?;
        assert_eq![networks.networks[0], nw];
        Ok(())
    }

    #[test]
    fn test_auto_no_ask_first_known2() -> Result<(), ErrBox> {
        let mut options = Options::default();
        options.auto_mode = AutoMode::AutoNoAsk;

        let networks = get_3_networks_last_known();
        let manual_selector = err_should_not_have_used_manual;
        let auto_selector = err_should_not_have_used_auto;
        let auto_no_ask_selector = select_first_known;
        let nw = select_network_impl(
            &options,
            &networks,
            manual_selector,
            auto_selector,
            auto_no_ask_selector,
        )?;
        assert_eq![networks.networks[2], nw];
        Ok(())
    }

    #[test]
    fn test_auto_fallback() -> Result<(), ErrBox> {
        let mut options = Options::default();
        options.auto_mode = AutoMode::Auto;

        let networks = get_3_unknown_networks();
        let manual_selector = select_first;
        let auto_selector = fail_to_select;
        let auto_no_ask_selector = err_should_not_have_used_auto_no_ask;
        let nw = select_network_impl(
            &options,
            &networks,
            manual_selector,
            auto_selector,
            auto_no_ask_selector,
        )?;
        assert_eq![networks.networks[0], nw];
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
