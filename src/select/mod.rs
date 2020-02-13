mod additional_selection_options;
mod external_selection_programs;
pub(crate) mod prompt_for_encryption_key;
mod text_format_for_display;

use additional_selection_options::*;
use external_selection_programs::*;

use crate::errors::*;
use crate::options::interfaces::*;
use crate::rerr;
use crate::sort_networks::SortedFilteredNetworks;
use crate::structs::*;

use std::fmt::Debug;
use std::str::FromStr;

impl<N: Identifiable + Selectable + Debug + Ord + Known + Clone> SortedFilteredNetworks<N> {
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

    pub(crate) fn select_network<O>(&self, options: &O) -> Result<N, RuwiError>
    where
        O: Global + AutoSelect,
    {
        self.select_network_impl(options, Self::prompt_user_for_selection::<O>)
    }

    fn select_network_impl<O, F>(&self, options: &O, manual_selector: F) -> Result<N, RuwiError>
    where
        O: Global + AutoSelect,
        F: FnOnce(&Self, &O) -> Result<N, RuwiError>,
    {
        let selected_network_res = match options.get_auto_mode() {
            AutoMode::Ask => manual_selector(self, options),
            AutoMode::KnownOrAsk => self
                .select_first_known(options)
                .or_else(|_| manual_selector(self, options)),
            AutoMode::KnownOrFail => self.select_first_known(options),
            AutoMode::First => self.select_first(options),
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

    fn prompt_user_for_selection<O>(&self, options: &O) -> Result<N, RuwiError>
    where
        O: Global,
    {
        let selector_output = self.run_manual_selector(options)?;

        if let Ok(selection_option) = SelectionOption::from_str(&selector_output) {
            match selection_option {
                SelectionOption::Refresh => {
                    eprintln!("[NOTE]: Refresh requested, running synchronous scan.");
                    Err(rerr!(RuwiErrorKind::RefreshRequested, "Refresh requested."))
                }
            }
        } else {
            let index = get_index_of_selected_item(&selector_output)?;

            self.get_networks()
                .get(index)
                .map(Clone::clone)
                .ok_or_else(|| {
                    rerr!(
                        RuwiErrorKind::NoNetworksFoundMatchingSelectionResult,
                        format!("No network matching {} found.", selector_output)
                    )
                })
        }
    }

    fn run_manual_selector<O>(&self, options: &O) -> Result<String, RuwiError>
    where
        O: Global,
    {
        self.run_manual_selector_impl(options, pass_tokens_to_selection_program::<O>)
    }

    fn run_manual_selector_impl<O, F>(&self, options: &O, selector: F) -> Result<String, RuwiError>
    where
        O: Global,
        F: FnOnce(&O, &[String]) -> Result<String, RuwiError>,
    {
        let selection_tokens = self.get_tokens_for_selection();
        selector(options, &selection_tokens).map(|x| x.trim().into())
    }
}

fn pass_tokens_to_selection_program<O>(
    options: &O,
    selection_tokens: &[String],
) -> Result<String, RuwiError>
where
    O: Global,
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

impl<N: Identifiable + Selectable + Debug + Ord + Known + Clone> SortedFilteredNetworks<N> {
    fn select_first_known<O>(&self, _options: &O) -> Result<N, RuwiError>
    where
        O: Global,
    {
        self.get_networks()
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

    fn select_first<O>(&self, _options: &O) -> Result<N, RuwiError>
    where
        O: Global,
    {
        self.get_networks()
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

    #[cfg(test)]
    fn select_last<O>(&self, _options: &O) -> Result<N, RuwiError>
    where
        O: Global,
    {
        self.get_networks()
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

    #[cfg(test)]
    fn select_refresh<O>(&self, _options: &O) -> Result<N, RuwiError>
    where
        O: Global,
    {
        dbg![&self];
        Err(rerr!(RuwiErrorKind::RefreshRequested, "Refresh requested."))
    }

    #[cfg(test)]
    fn err_should_not_have_used_manual<O>(&self, _opt: &O) -> Result<N, RuwiError>
    where
        O: Global,
    {
        dbg![&self];
        Err(rerr!(
            RuwiErrorKind::TestUsedManualWhenNotExpected,
            "Used manual selector in test when should not have!",
        ))
    }
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
        SortedFilteredNetworks::new(networks)
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

    #[test]
    fn test_manually_select_first_network() -> Result<(), RuwiError> {
        let options = WifiConnectOptions::default();
        assert_eq![options.get_auto_mode(), &AutoMode::Ask];
        let networks = get_3_unknown_networks();
        let nw = networks.select_network_impl(&options, SortedFilteredNetworks::select_first)?;
        assert_eq![networks.get_networks()[0], nw];
        Ok(())
    }

    #[test]
    fn test_manually_select_last_network() -> Result<(), RuwiError> {
        let options = WifiConnectOptions::default();
        assert_eq![options.get_auto_mode(), &AutoMode::Ask];
        let networks = get_3_unknown_networks();
        let nw = networks.select_network_impl(&options, SortedFilteredNetworks::select_last)?;
        assert_eq![networks.get_networks()[2], nw];
        Ok(())
    }

    #[test]
    fn test_fail_to_manually_select() {
        let options = WifiConnectOptions::default();
        assert_eq![options.get_auto_mode(), &AutoMode::Ask];
        let networks = get_3_unknown_networks();
        let res =
            networks.select_network_impl(&options, SortedFilteredNetworks::select_first_known);
        assert_eq![RuwiErrorKind::NoKnownNetworksFound, res.err().unwrap().kind];
    }

    #[test]
    fn test_auto_first_known() -> Result<(), RuwiError> {
        let options = WifiConnectOptions::builder()
            .wifi(WifiOptions::default())
            .auto_mode(AutoMode::KnownOrFail)
            .build();
        let networks = get_3_networks_last_known();
        let nw = networks.select_network_impl(
            &options,
            SortedFilteredNetworks::err_should_not_have_used_manual,
        )?;
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
        let nw = networks.select_network_impl(
            &options,
            SortedFilteredNetworks::err_should_not_have_used_manual,
        )?;
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
        let nw = networks.select_network_impl(
            &options,
            SortedFilteredNetworks::err_should_not_have_used_manual,
        )?;
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
        let nw = networks.select_network_impl(&options, SortedFilteredNetworks::select_first)?;
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
        let res = networks.select_network_impl(&options, SortedFilteredNetworks::select_refresh);
        assert_eq![RuwiErrorKind::RefreshRequested, res.err().unwrap().kind];
    }

    #[test]
    fn test_manual_selector_output() {
        let opts = WifiConnectOptions::default();
        let networks = get_3_unknown_networks();

        let run_without_whitespace = networks
            .run_manual_selector_impl(&opts, |_opts, names| {
                Ok(format!("{}", &names.first().unwrap()))
            })
            .unwrap();

        let run_with_whitespace = networks
            .run_manual_selector_impl(&opts, |_opts, names| {
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
        let networks = SortedFilteredNetworks::new(vec![
            AnnotatedWirelessNetwork::from_essid("FAKE NEWS LOL OK".to_string(), true, true),
            AnnotatedWirelessNetwork::from_essid("WOWWW OK FACEBO".to_string(), false, true),
            AnnotatedWirelessNetwork::from_essid("LOOK, DISCOURSE".to_string(), true, false),
            AnnotatedWirelessNetwork::from_essid("UWU MAMMMAAAAA".to_string(), false, false),
        ]);
        let tokens = networks.get_tokens_for_selection();
        for (i, (nw, token)) in networks.get_networks().iter().zip(tokens).enumerate() {
            let expected_token = format!("{}) {}", i, nw.get_display_string());
            assert_eq![expected_token, token];
        }
    }
}
