mod additional_options_for_manual_selection;
mod external_selection_programs;
mod get_index_of_selected_item;
pub(crate) mod prompt_for_encryption_key;
mod text_format_for_display;

use additional_options_for_manual_selection::get_possible_selection_options_as_strings;
use external_selection_programs::{run_dmenu, run_fzf, run_select_nocurses};
use get_index_of_selected_item::get_index_of_selected_item;

use crate::prelude::*;

use std::fmt::Debug;

// TODO: make a trait/API for selection
// TODO: make a trait/API for selectors

pub trait Selector: Sized + Debug
{
    type Item: Selectable + Identifiable + Clone + Debug + Known;
    fn get_networks(&self) -> &[Self::Item];

    fn get_network_tokens(&self) -> Vec<String> {
        self.get_networks()
            .iter()
            .enumerate()
            .map(|(i, x)| format!("{}) {}", i, x.get_display_string()))
            .collect()
    }
    fn get_tokens_for_selection(&self) -> Vec<String> {
        self.get_network_tokens()
            .into_iter()
            .chain(get_possible_selection_options_as_strings())
            .collect()
    }

    fn select_network<O>(&self, options: &O) -> Result<Self::Item, RuwiError>
    where
        O: Global + AutoSelect,
    {
        self.select_network_impl(options, Self::prompt_user_for_selection::<O>)
    }

    fn select_network_impl<O, F>(&self, options: &O, manual_selector: F) -> Result<Self::Item, RuwiError>
    where
        O: Global + AutoSelect,
        F: FnOnce(&Self, &O) -> Result<Self::Item, RuwiError>,
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
            Ok(nw) => eprintln!("[NOTE]: Selected network: \"{}\"", nw.get_public_name()),
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

    fn prompt_user_for_selection<O>(&self, options: &O) -> Result<Self::Item, RuwiError>
    where
        O: Global,
    {
        let selector_output = self.run_manual_selector(options)?;

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

    fn select_first_known<O>(&self, _options: &O) -> Result<Self::Item, RuwiError>
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

    fn select_first<O>(&self, _options: &O) -> Result<Self::Item, RuwiError>
    where
        O: Global,
    {
        self.get_networks()
            .iter()
            .next()
            .ok_or_else(|| {
                rerr!(
                    RuwiErrorKind::NoNetworksFoundWhenLookingForFirst,
                    "No networks found!"
                )
            })
            .map(Clone::clone)
    }

    #[cfg(test)]
    fn select_last<O>(&self, _options: &O) -> Result<Self::Item, RuwiError>
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
    fn select_refresh<O>(&self, _options: &O) -> Result<Self::Item, RuwiError>
    where
        O: Global,
    {
        dbg![&self];
        Err(rerr!(RuwiErrorKind::RefreshRequested, "Refresh requested."))
    }

    #[cfg(test)]
    fn err_should_not_have_used_manual<O>(&self, _opt: &O) -> Result<Self::Item, RuwiError>
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

fn pass_tokens_to_selection_program<O>(
    options: &O,
    selection_tokens: &[String],
) -> Result<String, RuwiError>
where
    O: Global,
{
    let target = options.get_post_parse_context().network_or_device.to_string();

    match options.get_selection_method() {
        SelectionMethod::NoCurses => run_select_nocurses(
            options,
            &format!("Select a {} (\"refresh\" or \".\" to rescan, Enter to select the top option): ", target),
            &selection_tokens,
        ),
        SelectionMethod::Dmenu => run_dmenu(options, 
            &format!("Select a {}: ", target),
            &selection_tokens),
        SelectionMethod::Fzf => run_fzf(
            options,
            &format!("Select a {} (ctrl-r or \"refresh\" to refresh results): ", target),
            &selection_tokens,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::additional_options_for_manual_selection::SelectionOption;
    use super::*;
    use crate::options::wifi::connect::WifiConnectOptions;
    use crate::options::wifi::WifiOptions;
    use crate::sort_networks::SortedFilteredNetworks;
    use crate::structs::AnnotatedWirelessNetwork;
    use crate::strum::AsStaticRef;

    static FIRST_NW_NAME: &str = "FIRSTNWLOL";
    static SECND_NW_NAME: &str = "SECNDNWWUT";
    static THIRD_NW_NAME: &str = "THIRDNWOKK";

    fn get_3_networks() -> SortedFilteredNetworks<AnnotatedWirelessNetwork> {
        let networks = [FIRST_NW_NAME, SECND_NW_NAME, THIRD_NW_NAME]
            .iter()
            .map(|name| AnnotatedWirelessNetwork::from_essid_only(name))
            .collect::<Vec<AnnotatedWirelessNetwork>>();
        SortedFilteredNetworks::new(networks)
    }

    fn get_3_unknown_networks() -> SortedFilteredNetworks<AnnotatedWirelessNetwork> {
        get_3_networks()
    }

    fn get_3_networks_first_known() -> SortedFilteredNetworks<AnnotatedWirelessNetwork> {
        let mut networks = get_3_networks();
        networks.get_networks_mut()[0]
            .set_service_identifier_for_tests(NetworkingServiceIdentifier::netctl_nw("some_id"));
        networks
    }

    fn get_3_networks_last_known() -> SortedFilteredNetworks<AnnotatedWirelessNetwork> {
        let mut networks = get_3_networks();
        networks.get_networks_mut()[2]
            .set_service_identifier_for_tests(NetworkingServiceIdentifier::netctl_nw("some_id"));
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
    fn test_auto_no_ask_unknown() -> Result<(), RuwiError> {
        let options = WifiConnectOptions::builder()
            .wifi(WifiOptions::default())
            .auto_mode(AutoMode::KnownOrFail)
            .build();
        let networks = get_3_unknown_networks();
        let nw = networks.select_network_impl(
            &options,
            SortedFilteredNetworks::err_should_not_have_used_manual,
        );
        assert_eq![RuwiErrorKind::NoKnownNetworksFound, nw.err().unwrap().kind];
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
            .run_manual_selector_impl(&opts, |_opts, names| Ok(names.first().unwrap().to_string()))
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
    fn test_get_tokens_for_selection() {
        let networks = SortedFilteredNetworks::new(vec![
            AnnotatedWirelessNetwork::builder()
                .essid("FAKE NEWS LOL OK")
                .service_identifier(NetworkingServiceIdentifier::netctl_nw("some_id"))
                .is_encrypted(true)
                .build(),
            AnnotatedWirelessNetwork::builder()
                .essid("WOWWW OK FACEBO")
                .service_identifier(None)
                .is_encrypted(true)
                .build(),
            AnnotatedWirelessNetwork::builder()
                .essid("LOOK, DISCOURSE")
                .service_identifier(NetworkingServiceIdentifier::netctl_nw("some_id"))
                .is_encrypted(false)
                .build(),
            AnnotatedWirelessNetwork::builder()
                .essid("UWU MAMMMAAAAA")
                .service_identifier(None)
                .is_encrypted(false)
                .build(),
        ]);
        let tokens = networks.get_tokens_for_selection();
        for (i, (nw, token)) in networks.get_networks().iter().zip(tokens).enumerate() {
            let expected_token = format!("{}) {}", i, nw.get_display_string());
            assert_eq![expected_token, token];
        }
    }
}
