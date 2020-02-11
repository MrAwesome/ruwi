// TODO: abstract away this functionality to not be wifi-specific
use crate::errors::*;
use crate::options::interfaces::*;
use crate::rerr;
use crate::select_utils::*;
use crate::sort_networks::SortedFilteredNetworks;
use crate::structs::*;

use crate::strum_utils::possible_vals;
use std::str::FromStr;
use std::fmt::Debug;

fn get_possible_selection_options_as_strings() -> Vec<String> {
    possible_vals::<SelectionOption, _>()
        .iter()
        .map(|&x| x.into())
        .collect()
}

impl<T: Ord + Identifiable + Selectable + Clone> SortedFilteredNetworks<T> {
    pub fn get_tokens_for_selection(&self) -> Vec<String> {
        self.get_network_tokens()
            .into_iter()
            .chain(get_possible_selection_options_as_strings())
            .collect()
    }

    pub fn get_network_tokens(&self) -> Vec<String> {
        self.get_networks()
            .iter()
            .enumerate()
            .map(|(i, x)| format!("{}) {}", i, x.get_display_string()))
            .collect()
    }
}

pub(crate) fn select_network<O, N, T>(
    options: &O,
    networks: &SortedFilteredNetworks<N>,
) -> Result<N, RuwiError>
where
    O: Global + AutoSelect,
    N: Identifiable + Selectable + Ord + Debug + Annotated<T> + Clone,
{
    select_network_impl(options, networks, prompt_user_for_selection)
}

fn prompt_user_for_selection<O, N>(
    options: &O,
    networks: &SortedFilteredNetworks<N>,
) -> Result<N, RuwiError>
where
    O: Global,
    N: Identifiable + Selectable + Ord + Clone,
{
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

        networks.get_networks().get(index).map(Clone::clone).ok_or_else(|| {
            rerr!(
                RuwiErrorKind::NoNetworksFoundMatchingSelectionResult,
                format!("No network matching {} found.", selector_output)
            )
        })
    }
}

fn run_manual_selector<O, N>(
    options: &O,
    networks: &SortedFilteredNetworks<N>,
) -> Result<String, RuwiError>
where
    O: Global,
    N: Identifiable + Selectable + Ord + Clone,
{
    run_manual_selector_impl(options, networks, pass_tokens_to_selection_program::<O, N>)
}

fn run_manual_selector_impl<O, N, F>(
    options: &O,
    networks: &SortedFilteredNetworks<N>,
    selector: F,
) -> Result<String, RuwiError>
where
    O: Global,
    N: Identifiable + Selectable + Ord + Clone,
    F: FnOnce(&O, &[String]) -> Result<String, RuwiError>,
{
    let selection_tokens = networks.get_tokens_for_selection();
    selector(options, &selection_tokens).map(|x| x.trim().into())
}

fn pass_tokens_to_selection_program<O, N>(
    options: &O,
    selection_tokens: &[String],
) -> Result<String, RuwiError>
where
    O: Global,
    N: Identifiable + Selectable + Ord,
{
    match options.get_selection_method() {
        SelectionMethod::Dmenu => run_dmenu(options, "Select a network: ", &selection_tokens),
        SelectionMethod::Fzf => run_fzf(
            options,
            "Select a network (ctrl-r or \"refresh\" to refresh results): ",
            &selection_tokens,
        ),
    }
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

fn select_first_known<O, N, T>(
    _options: &O,
    networks: &SortedFilteredNetworks<N>,
) -> Result<N, RuwiError>
where
    O: Global,
    N: Identifiable + Selectable + Ord + Annotated<T> + Clone,
{
    networks
        .get_networks()
        .iter()
        .find(|nw| nw.is_known())
        .ok_or_else(|| {
            rerr!(
                RuwiErrorKind::NoKnownNetworksFound,
                "No known networks found!"
            )
        })
        .map(Clone::clone)
}

fn select_first<O, N>(_options: &O, networks: &SortedFilteredNetworks<N>) -> Result<N, RuwiError>
where
    O: Global,
    N: Identifiable + Selectable + Ord + Clone,
{
    networks
        .get_networks()
        .iter()
        .next()
        .ok_or_else(|| {
            rerr!(
                RuwiErrorKind::TestNoNetworksFoundWhenLookingForFirst,
                "No networks found!"
            )
        })
        .map(Clone::clone)
}

fn select_network_impl<'a, 'b, O, N, F, T>(
    options: &'a O,
    networks: &'b SortedFilteredNetworks<N>,
    manual_selector: F,
) -> Result<N, RuwiError>
where
    O: Global + AutoSelect,
    N: Identifiable + Selectable + Ord + Annotated<T> + Clone + Debug,
    F: FnOnce(&'a O, &'b SortedFilteredNetworks<N>) -> Result<N, RuwiError>,
{
    let selected_network_res = match options.get_auto_mode() {
        AutoMode::Ask => manual_selector(options, networks),
        AutoMode::KnownOrAsk => {
            select_first_known(options, networks).or_else(|_| manual_selector(options, networks))
        }
        AutoMode::KnownOrFail => select_first_known(options, networks),
        AutoMode::First => select_first(options, networks),
    };

    match &selected_network_res {
        // TODO: should there be a less machine-specific version of get_identifier?
        Ok(nw) => eprintln!("[NOTE]: Selected network: \"{}\"", nw.get_identifier()),
        Err(_) => {
            if options.get_auto_mode() == &AutoMode::KnownOrFail {
                eprintln!(
                    "[ERR]: Failed to find a known network in `known_or_fail` mode. Will exit now."
                );
            }
        }
    }

    if options.d() {
        dbg![&selected_network_res];
    }
    selected_network_res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::wifi::connect::WifiConnectOptions;
    use crate::options::wifi::WifiOptions;
    use crate::strum::AsStaticRef;

    static FIRST_NW_NAME: &str = "FIRSTNWLOL";
    static SECND_NW_NAME: &str = "SECNDNWWUT";
    static THIRD_NW_NAME: &str = "THIRDNWOKK";

    fn get_3_networks() -> SortedFilteredNetworks<AnnotatedWirelessNetwork> {
        let networks = [FIRST_NW_NAME, SECND_NW_NAME, THIRD_NW_NAME]
            .iter()
            .map(|name| AnnotatedWirelessNetwork::from_essid((*name).to_string(), false, false))
            .collect::<Vec<AnnotatedWirelessNetwork>>();
        SortedFilteredNetworks::new(&networks)
    }

    fn get_3_unknown_networks() -> SortedFilteredNetworks<AnnotatedWirelessNetwork> {
        get_3_networks()
    }

    fn get_3_networks_first_known() -> SortedFilteredNetworks<AnnotatedWirelessNetwork> {
        let mut networks = get_3_networks();
        networks.get_networks_mut()[0].known = true;
        networks
    }

    fn get_3_networks_last_known() -> SortedFilteredNetworks<AnnotatedWirelessNetwork> {
        let mut networks = get_3_networks();
        networks.get_networks_mut()[2].known = true;
        networks
    }

    fn select_last<O, N>(_options: &O, networks: &SortedFilteredNetworks<N>) -> Result<N, RuwiError>
    where
        O: Global,
        N: Identifiable + Selectable + Ord + Clone,
    {
        networks
            .get_networks()
            .iter()
            .last()
            .ok_or_else(|| {
                rerr!(
                    RuwiErrorKind::TestNoNetworksFoundWhenLookingForLast,
                    "No networks found!"
                )
            })
            .map(Clone::clone)
    }

    fn select_refresh<O, N>(_options: &O, _networks: &SortedFilteredNetworks<N>) -> Result<N, RuwiError>
    where
        O: Global,
        N: Identifiable + Selectable + Ord + Clone,
    {
        Err(rerr!(RuwiErrorKind::RefreshRequested, "Refresh requested."))
    }

    fn err_should_not_have_used_manual<O, N>(
        _opt: &O,
        _nw: &SortedFilteredNetworks<N>,
    ) -> Result<N, RuwiError>
    where
        O: Global,
        N: Identifiable + Selectable + Ord + Clone,
    {
        Err(rerr!(
            RuwiErrorKind::TestUsedManualWhenNotExpected,
            "Used manual selector in test when should not have!",
        ))
    }

    #[test]
    fn test_manually_select_first_network() -> Result<(), RuwiError> {
        let options = WifiConnectOptions::default();
        assert_eq![options.get_auto_mode(), &AutoMode::Ask];
        let networks = get_3_unknown_networks();
        let nw = select_network_impl(&options, &networks, select_first)?;
        assert_eq![networks.get_networks()[0], nw];
        Ok(())
    }

    #[test]
    fn test_manually_select_last_network() -> Result<(), RuwiError> {
        let options = WifiConnectOptions::default();
        assert_eq![options.get_auto_mode(), &AutoMode::Ask];
        let networks = get_3_unknown_networks();
        let nw = select_network_impl(&options, &networks, select_last)?;
        assert_eq![networks.get_networks()[2], nw];
        Ok(())
    }

    #[test]
    fn test_fail_to_manually_select() {
        let options = WifiConnectOptions::default();
        assert_eq![options.get_auto_mode(), &AutoMode::Ask];
        let networks = get_3_unknown_networks();
        let res = select_network_impl(&options, &networks, select_first_known);
        assert_eq![RuwiErrorKind::NoKnownNetworksFound, res.err().unwrap().kind];
    }

    #[test]
    fn test_auto_first_known() -> Result<(), RuwiError> {
        let options = WifiConnectOptions::builder()
            .wifi(WifiOptions::default())
            .auto_mode(AutoMode::KnownOrFail)
            .build();
        let networks = get_3_networks_last_known();
        let nw = select_network_impl(&options, &networks, err_should_not_have_used_manual)?;
        assert_eq![networks.get_networks()[2], nw];
        Ok(())
    }

    #[test]
    fn test_auto_no_ask_first_known() -> Result<(), RuwiError> {
        let options = WifiConnectOptions::builder()
            .wifi(WifiOptions::default())
            .auto_mode(AutoMode::KnownOrFail)
            .build();
        let networks = get_3_networks_first_known();
        let nw = select_network_impl(&options, &networks, err_should_not_have_used_manual)?;
        assert_eq![networks.get_networks()[0], nw];
        Ok(())
    }

    #[test]
    fn test_auto_no_ask_first_known2() -> Result<(), RuwiError> {
        let options = WifiConnectOptions::builder()
            .wifi(WifiOptions::default())
            .auto_mode(AutoMode::KnownOrFail)
            .build();
        let networks = get_3_networks_last_known();
        let nw = select_network_impl(&options, &networks, err_should_not_have_used_manual)?;
        assert_eq![networks.get_networks()[2], nw];
        Ok(())
    }

    #[test]
    fn test_auto_fallback() -> Result<(), RuwiError> {
        let options = WifiConnectOptions::builder()
            .wifi(WifiOptions::default())
            .auto_mode(AutoMode::KnownOrAsk)
            .build();
        let networks = get_3_unknown_networks();
        let nw = select_network_impl(&options, &networks, select_first)?;
        assert_eq![networks.get_networks()[0], nw];
        Ok(())
    }

    #[test]
    fn test_manually_refresh() {
        let options = WifiConnectOptions::default();
        assert_eq![options.get_auto_mode(), &AutoMode::Ask];
        let networks = get_3_unknown_networks();
        assert![networks
            .get_tokens_for_selection()
            .contains(&SelectionOption::Refresh.as_static().into())];
        let res = select_network_impl(&options, &networks, select_refresh);
        assert_eq![RuwiErrorKind::RefreshRequested, res.err().unwrap().kind];
    }

    #[test]
    fn test_manual_selector_output() {
        let opts = WifiConnectOptions::default();
        let networks = get_3_unknown_networks();

        let run_without_whitespace = run_manual_selector_impl(&opts, &networks, |_opts, names| {
            Ok(format!("{}", &names.first().unwrap()))
        })
        .unwrap();

        let run_with_whitespace = run_manual_selector_impl(&opts, &networks, |_opts, names| {
            Ok(format!(
                " \n\n  \n {}   \n\n   \t   \n",
                &names.first().unwrap()
            ))
        })
        .unwrap();

        dbg!(&run_without_whitespace, &run_with_whitespace);
        assert_eq!(run_without_whitespace, run_with_whitespace);
    }

    #[test]
    fn test_get_indices() -> Result<(), RuwiError> {
        let test_cases: Vec<(&str, Result<usize, RuwiError>)> = vec![
            ("1) jfdlskajfdlksa", Ok(1)),
            ("0) jfdlskajfdlksa", Ok(0)),
            ("22) jfdlskajfdlksa", Ok(22)),
            ("69) 54) jfdlskajfdlksa", Ok(69)),
            ("4000) jfdlskajfdlksa", Ok(4000)),
            ("4000000000) jfdlskajfdlksa", Ok(4_000_000_000)),
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
        let networks = SortedFilteredNetworks::new(
            &[
                AnnotatedWirelessNetwork::from_essid("FAKE NEWS LOL OK".to_string(), true, true),
                AnnotatedWirelessNetwork::from_essid("WOWWW OK FACEBO".to_string(), false, true),
                AnnotatedWirelessNetwork::from_essid("LOOK, DISCOURSE".to_string(), true, false),
                AnnotatedWirelessNetwork::from_essid("UWU MAMMMAAAAA".to_string(), false, false),
            ]
        );
        let tokens = networks.get_tokens_for_selection();
        for (i, (nw, token)) in networks.get_networks().iter().zip(tokens).enumerate() {
            let expected_token = format!("{}) {}", i, nw.get_display_string());
            assert_eq![expected_token, token];
        }
    }
}
