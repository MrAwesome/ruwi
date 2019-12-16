use crate::rerr;
use crate::select_utils::*;
use crate::structs::*;

use crate::strum_utils::possible_vals;
use std::str::FromStr;

fn get_possible_selection_options_as_strings() -> Vec<String> {
    possible_vals::<SelectionOption, _>()
        .iter()
        .map(|&x| x.into())
        .collect()
}

impl SortedUniqueNetworks {
    pub fn get_tokens_for_selection(&self) -> Vec<String> {
        self.get_network_tokens()
            .into_iter()
            .chain(get_possible_selection_options_as_strings())
            .collect()
    }

    pub fn get_network_tokens(&self) -> Vec<String> {
        self.networks
            .iter()
            .enumerate()
            .map(|(i, x)| format!("{}) {}", i, x.get_display_string()))
            .collect()
    }
}

pub(crate) fn select_network(
    options: &Options,
    networks: &SortedUniqueNetworks,
) -> Result<AnnotatedWirelessNetwork, RuwiError> {
    select_network_impl(options, networks, prompt_user_for_selection)
}

fn prompt_user_for_selection(
    options: &Options,
    networks: &SortedUniqueNetworks,
) -> Result<AnnotatedWirelessNetwork, RuwiError> {
    let selector_output = run_manual_selector(options, networks)?;

    if let Ok(selection_option) = SelectionOption::from_str(&selector_output) {
        match selection_option {
            SelectionOption::Refresh => {
                eprintln!("[NOTE]: Refresh requested, running synchronous scan.");
                Err(rerr!(RuwiErrorKind::RefreshRequested, "Refresh requested."))
            }
        }
    } else {
        let index = get_index_of_selected_item(&selector_output)?;

        networks.networks.get(index).cloned().ok_or_else(|| {
            rerr!(
                RuwiErrorKind::NoNetworksFoundMatchingSelectionResult,
                format!("No network matching {} found.", selector_output)
            )
        })
    }
}
fn run_manual_selector(
    options: &Options,
    networks: &SortedUniqueNetworks,
) -> Result<String, RuwiError> {
    let selection_tokens = networks.get_tokens_for_selection();
    match &options.selection_method {
        SelectionMethod::Dmenu => run_dmenu(
            &options,
            &"Select a network: ".to_string(),
            selection_tokens,
        ),
        SelectionMethod::Fzf => run_fzf(
            &options,
            &"Select a network (ctrl-r or \"refresh\" to refresh results): ".to_string(),
            selection_tokens,
        ),
    }
    // TODO: unit test that this trim happens, it is very important.
    .map(|x| x.trim().into())
}

fn get_index_of_selected_item(line: &str) -> Result<usize, RuwiError> {
    line.split(") ")
        .next()
        .ok_or_else(|| get_line_parse_err(line))?
        .parse::<usize>()
        .or_else(|_| Err(get_line_parse_err(line)))
}

fn get_line_parse_err(line: &str) -> RuwiError {
    rerr!(
        RuwiErrorKind::FailedToParseSelectedLine,
        format!("Failed to parse line {}", line)
    )
}

fn select_first_known(
    _options: &Options,
    networks: &SortedUniqueNetworks,
) -> Result<AnnotatedWirelessNetwork, RuwiError> {
    networks
        .networks
        .iter()
        .find(|nw| nw.known)
        .ok_or_else(|| {
            rerr!(
                RuwiErrorKind::NoKnownNetworksFound,
                "No known networks found!"
            )
        })
        .map(|x| x.clone())
}

fn select_first(
    _options: &Options,
    networks: &SortedUniqueNetworks,
) -> Result<AnnotatedWirelessNetwork, RuwiError> {
    networks
        .networks
        .iter()
        .next()
        .ok_or_else(|| {
            rerr!(
                RuwiErrorKind::TestNoNetworksFoundWhenLookingForFirst,
                "No networks found!"
            )
        })
        .map(|x| x.clone())
}

fn select_network_impl<'a, 'b, F>(
    options: &'a Options,
    networks: &'b SortedUniqueNetworks,
    manual_selector: F,
) -> Result<AnnotatedWirelessNetwork, RuwiError>
where
    F: FnOnce(&'a Options, &'b SortedUniqueNetworks) -> Result<AnnotatedWirelessNetwork, RuwiError>,
{
    let selected_network_res = match &options.auto_mode {
        AutoMode::Ask => manual_selector(options, networks),
        AutoMode::KnownOrAsk => {
            select_first_known(options, networks).or_else(|_| manual_selector(options, networks))
        }
        AutoMode::KnownOrFail => select_first_known(options, networks),
        AutoMode::First => select_first(options, networks),
    };

    // TODO: sensible error messages for when auto no ask fails

    if let Ok(nw) = &selected_network_res {
        eprintln!("[NOTE]: Selected network: \"{}\"", nw.essid);
    }

    if options.debug {
        dbg![&selected_network_res];
    }
    selected_network_res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strum::AsStaticRef;

    static FIRST_NW_NAME: &str = "FIRSTNWLOL";
    static SECND_NW_NAME: &str = "SECNDNWWUT";
    static THIRD_NW_NAME: &str = "THIRDNWOKK";

    fn get_3_networks() -> SortedUniqueNetworks {
        let networks = vec![FIRST_NW_NAME, SECND_NW_NAME, THIRD_NW_NAME]
            .iter()
            .map(|name| AnnotatedWirelessNetwork {
                essid: name.to_string(),
                ..Default::default()
            })
            .collect();
        SortedUniqueNetworks { networks }
    }

    fn get_3_unknown_networks() -> SortedUniqueNetworks {
        get_3_networks()
    }

    fn get_3_networks_first_known() -> SortedUniqueNetworks {
        let mut networks = get_3_networks();
        networks.networks[0].known = true;
        networks
    }

    fn get_3_networks_last_known() -> SortedUniqueNetworks {
        let mut networks = get_3_networks();
        networks.networks[2].known = true;
        networks
    }

    fn select_last(
        _options: &Options,
        networks: &SortedUniqueNetworks,
    ) -> Result<AnnotatedWirelessNetwork, RuwiError> {
        networks
            .networks
            .iter()
            .last()
            .ok_or_else(|| {
                rerr!(
                    RuwiErrorKind::TestNoNetworksFoundWhenLookingForLast,
                    "No networks found!"
                )
            })
            .map(|x| x.clone())
    }

    fn select_refresh(
        _options: &Options,
        _networks: &SortedUniqueNetworks,
    ) -> Result<AnnotatedWirelessNetwork, RuwiError> {
        Err(rerr!(RuwiErrorKind::RefreshRequested, "Refresh requested."))
    }

    fn err_should_not_have_used_manual(
        _opt: &Options,
        _nw: &SortedUniqueNetworks,
    ) -> Result<AnnotatedWirelessNetwork, RuwiError> {
        Err(rerr!(
            RuwiErrorKind::TestUsedManualWhenNotExpected,
            "Used manual selector in test when should not have!",
        ))
    }

    #[test]
    fn test_manually_select_first_network() -> Result<(), RuwiError> {
        let options = Options::default();
        assert_eq![options.auto_mode, AutoMode::Ask];
        let networks = get_3_unknown_networks();
        let nw = select_network_impl(&options, &networks, select_first)?;
        assert_eq![networks.networks[0], nw];
        Ok(())
    }

    #[test]
    fn test_manually_select_last_network() -> Result<(), RuwiError> {
        let options = Options::default();
        assert_eq![options.auto_mode, AutoMode::Ask];
        let networks = get_3_unknown_networks();
        let nw = select_network_impl(&options, &networks, select_last)?;
        assert_eq![networks.networks[2], nw];
        Ok(())
    }

    #[test]
    fn test_fail_to_manually_select() {
        let options = Options::default();
        assert_eq![options.auto_mode, AutoMode::Ask];
        let networks = get_3_unknown_networks();
        let res = select_network_impl(&options, &networks, select_first_known);
        assert_eq![RuwiErrorKind::NoKnownNetworksFound, res.err().unwrap().kind];
    }

    #[test]
    fn test_auto_first_known() -> Result<(), RuwiError> {
        let mut options = Options::default();
        options.auto_mode = AutoMode::KnownOrFail;

        let networks = get_3_networks_last_known();
        let nw = select_network_impl(&options, &networks, err_should_not_have_used_manual)?;
        assert_eq![networks.networks[2], nw];
        Ok(())
    }

    #[test]
    fn test_auto_no_ask_first_known() -> Result<(), RuwiError> {
        let mut options = Options::default();
        options.auto_mode = AutoMode::KnownOrFail;

        let networks = get_3_networks_first_known();
        let nw = select_network_impl(&options, &networks, err_should_not_have_used_manual)?;
        assert_eq![networks.networks[0], nw];
        Ok(())
    }

    #[test]
    fn test_auto_no_ask_first_known2() -> Result<(), RuwiError> {
        let mut options = Options::default();
        options.auto_mode = AutoMode::KnownOrFail;

        let networks = get_3_networks_last_known();
        let nw = select_network_impl(&options, &networks, err_should_not_have_used_manual)?;
        assert_eq![networks.networks[2], nw];
        Ok(())
    }

    #[test]
    fn test_auto_fallback() -> Result<(), RuwiError> {
        let mut options = Options::default();
        options.auto_mode = AutoMode::KnownOrAsk;

        let networks = get_3_unknown_networks();
        let nw = select_network_impl(&options, &networks, select_first)?;
        assert_eq![networks.networks[0], nw];
        Ok(())
    }

    #[test]
    fn test_manually_refresh() {
        let options = Options::default();
        assert_eq![options.auto_mode, AutoMode::Ask];
        let networks = get_3_unknown_networks();
        assert![networks
            .get_tokens_for_selection()
            .contains(&SelectionOption::Refresh.as_static().into())];
        let res = select_network_impl(&options, &networks, select_refresh);
        assert_eq![RuwiErrorKind::RefreshRequested, res.err().unwrap().kind];
    }

    #[test]
    fn test_get_indices() -> Result<(), RuwiError> {
        let test_cases: Vec<(&str, Result<usize, RuwiError>)> = vec![
            ("1) jfdlskajfdlksa", Ok(1)),
            ("0) jfdlskajfdlksa", Ok(0)),
            ("22) jfdlskajfdlksa", Ok(22)),
            ("69) 54) jfdlskajfdlksa", Ok(69)),
            ("4000) jfdlskajfdlksa", Ok(4000)),
            ("4000000000) jfdlskajfdlksa", Ok(4000000000)),
            ("-12) negawifi", Err(get_line_parse_err("-12) negawifi"))),
            ("jf jfjf", Err(get_line_parse_err("jf jfjf"))),
            ("!@&*(#@!", Err(get_line_parse_err("!@&*(#@!"))),
        ];

        for (line, res) in test_cases {
            match get_index_of_selected_item(line) {
                Ok(val) => assert_eq![res?, val],
                Err(err) => assert_eq![res.err().unwrap().kind, err.kind],
            }
        }
        Ok(())
    }

    #[test]
    fn test_get_tokens_for_selection() {
        let networks = SortedUniqueNetworks {
            networks: vec![
                AnnotatedWirelessNetwork::from_essid("FAKE NEWS LOL OK".to_string(), true, true),
                AnnotatedWirelessNetwork::from_essid("WOWWW OK FACEBO".to_string(), false, true),
                AnnotatedWirelessNetwork::from_essid("LOOK, DISCOURSE".to_string(), true, false),
                AnnotatedWirelessNetwork::from_essid("UWU MAMMMAAAAA".to_string(), false, false),
            ],
        };
        let tokens = networks.get_tokens_for_selection();
        for i in 0..networks.networks.len() {
            let expected_token = format!("{}) {}", i, &networks.networks[i].get_display_string());
            assert_eq![expected_token, tokens[i]];
        }
    }
}
