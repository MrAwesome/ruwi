use crate::errbox;
#[cfg(not(test))]
use crate::select::*;
use crate::structs::*;

const NO_KNOWN_NETWORKS_FOUND: &str = "No known networks found!";

impl SortedUniqueNetworks {
    pub fn get_tokens_for_selection(&self) -> Vec<String> {
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
) -> Result<AnnotatedWirelessNetwork, ErrBox> {
    select_network_impl(
        options,
        networks,
        prompt_user_for_selection,
        auto_select_network_method,
        auto_no_ask_select_network_method,
    )
}

#[cfg(not(test))]
fn prompt_user_for_selection(
    options: &Options,
    networks: &SortedUniqueNetworks,
) -> Result<AnnotatedWirelessNetwork, ErrBox> {
    let selection_tokens = networks.get_tokens_for_selection();
    let selected_network_line = match &options.selection_method {
        SelectionMethod::Dmenu => {
            run_dmenu(&options, &"Select a network:".to_string(), selection_tokens)
        }
        SelectionMethod::Fzf => {
            run_fzf(&options, &"Select a network:".to_string(), selection_tokens)
        }
    }?;

    let index = get_index_of_selected_item(&selected_network_line)?;

    networks
        .networks
        .iter()
        .nth(index)
        .map(|x| x.clone())
        .ok_or_else(|| {
            errbox!(format!(
                "No network matching {} found.",
                selected_network_line
            ))
        })
}

// TODO: unit test this function
fn get_index_of_selected_item(line: &str) -> Result<usize, ErrBox> {
    line.split(") ")
        .next()
        .ok_or_else(|| get_line_parse_err(line))?
        .parse::<usize>()
        .or_else(|_| Err(get_line_parse_err(line)))
}

fn get_line_parse_err(line: &str) -> ErrBox {
    errbox!(format!("Failed to parse line {}", line))
}

#[cfg(test)]
fn prompt_user_for_selection(
    _options: &Options,
    _networks: &SortedUniqueNetworks,
) -> Result<AnnotatedWirelessNetwork, ErrBox> {
    panic!("Should not use this function in tests!");
}

fn auto_select_network_method(
    options: &Options,
    networks: &SortedUniqueNetworks,
) -> Result<AnnotatedWirelessNetwork, ErrBox> {
    select_first_known(options, networks)
}

fn auto_no_ask_select_network_method(
    options: &Options,
    networks: &SortedUniqueNetworks,
) -> Result<AnnotatedWirelessNetwork, ErrBox> {
    select_first_known(options, networks)
}

fn select_first_known(
    _options: &Options,
    networks: &SortedUniqueNetworks,
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
    networks: &'b SortedUniqueNetworks,
    manual_selector: F,
    auto_selector: G,
    auto_no_ask_selector: H,
) -> Result<AnnotatedWirelessNetwork, ErrBox>
where
    F: FnOnce(&'a Options, &'b SortedUniqueNetworks) -> Result<AnnotatedWirelessNetwork, ErrBox>,
    G: FnOnce(&'a Options, &'b SortedUniqueNetworks) -> Result<AnnotatedWirelessNetwork, ErrBox>,
    H: FnOnce(&'a Options, &'b SortedUniqueNetworks) -> Result<AnnotatedWirelessNetwork, ErrBox>,
{
    let selected_network = match &options.auto_mode {
        AutoMode::None => manual_selector(options, networks),
        AutoMode::Auto => {
            auto_selector(options, networks).or_else(|_| manual_selector(options, networks))
        }
        AutoMode::AutoNoAsk => auto_no_ask_selector(options, networks),
    };

    // TODO: sensible error messages for when auto no ask fails
    // TODO: eprintln! that you've found a network and what the ssid is

    if options.debug {
        dbg![&selected_network];
    }
    selected_network
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

    fn err_should_not_have_used_manual(
        _opt: &Options,
        _nw: &SortedUniqueNetworks,
    ) -> Result<AnnotatedWirelessNetwork, ErrBox> {
        Err(errbox!(USED_MANUAL_WHEN_NOT_EXPECTED))
    }

    fn err_should_not_have_used_auto(
        _opt: &Options,
        _nw: &SortedUniqueNetworks,
    ) -> Result<AnnotatedWirelessNetwork, ErrBox> {
        Err(errbox!(USED_AUTO_WHEN_NOT_EXPECTED))
    }

    fn err_should_not_have_used_auto_no_ask(
        _opt: &Options,
        _nw: &SortedUniqueNetworks,
    ) -> Result<AnnotatedWirelessNetwork, ErrBox> {
        Err(errbox!(USED_AUTO_NO_ASK_WHEN_NOT_EXPECTED))
    }

    fn select_first(
        _options: &Options,
        networks: &SortedUniqueNetworks,
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
        networks: &SortedUniqueNetworks,
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
        _networks: &SortedUniqueNetworks,
    ) -> Result<AnnotatedWirelessNetwork, ErrBox> {
        Err(errbox!("No networks found!"))
    }

    #[test]
    fn test_manually_select_first_network() -> Result<(), ErrBox> {
        let options = Options::default();
        assert_eq![options.auto_mode, AutoMode::None];
        let networks = get_3_unknown_networks();
        let nw = select_network_impl(
            &options,
            &networks,
            select_first,
            err_should_not_have_used_auto,
            err_should_not_have_used_auto_no_ask,
        )?;
        assert_eq![networks.networks[0], nw];
        Ok(())
    }

    #[test]
    fn test_manually_select_last_network() -> Result<(), ErrBox> {
        let options = Options::default();
        assert_eq![options.auto_mode, AutoMode::None];
        let networks = get_3_unknown_networks();
        let nw = select_network_impl(
            &options,
            &networks,
            select_last,
            err_should_not_have_used_auto,
            err_should_not_have_used_auto_no_ask,
        )?;
        assert_eq![networks.networks[2], nw];
        Ok(())
    }

    #[test]
    fn test_fail_to_manually_select() -> Result<(), ErrBox> {
        let options = Options::default();
        assert_eq![options.auto_mode, AutoMode::None];
        let networks = get_3_unknown_networks();
        let nw = select_network_impl(
            &options,
            &networks,
            select_first_known,
            err_should_not_have_used_auto,
            err_should_not_have_used_auto_no_ask,
        );
        match nw {
            Ok(_) => panic!(),
            Err(err) => assert_eq![err.description(), NO_KNOWN_NETWORKS_FOUND],
        };
        Ok(())
    }

    #[test]
    fn test_auto_first_known() -> Result<(), ErrBox> {
        let mut options = Options::default();
        options.auto_mode = AutoMode::AutoNoAsk;

        let networks = get_3_networks_last_known();
        let nw = select_network_impl(
            &options,
            &networks,
            err_should_not_have_used_manual,
            err_should_not_have_used_auto,
            select_first_known,
        )?;
        assert_eq![networks.networks[2], nw];
        Ok(())
    }

    #[test]
    fn test_auto_no_ask_first_known() -> Result<(), ErrBox> {
        let mut options = Options::default();
        options.auto_mode = AutoMode::AutoNoAsk;

        let networks = get_3_networks_first_known();
        let nw = select_network_impl(
            &options,
            &networks,
            err_should_not_have_used_manual,
            err_should_not_have_used_auto,
            select_first_known,
        )?;
        assert_eq![networks.networks[0], nw];
        Ok(())
    }

    #[test]
    fn test_auto_no_ask_first_known2() -> Result<(), ErrBox> {
        let mut options = Options::default();
        options.auto_mode = AutoMode::AutoNoAsk;

        let networks = get_3_networks_last_known();
        let nw = select_network_impl(
            &options,
            &networks,
            err_should_not_have_used_manual,
            err_should_not_have_used_auto,
            select_first_known,
        )?;
        assert_eq![networks.networks[2], nw];
        Ok(())
    }

    #[test]
    fn test_auto_fallback() -> Result<(), ErrBox> {
        let mut options = Options::default();
        options.auto_mode = AutoMode::Auto;

        let networks = get_3_unknown_networks();
        let nw = select_network_impl(
            &options,
            &networks,
            select_first,
            fail_to_select,
            err_should_not_have_used_auto_no_ask,
        )?;
        assert_eq![networks.networks[0], nw];
        Ok(())
    }

    #[test]
    fn test_get_indices() -> Result<(), ErrBox> {
        let test_cases: Vec<(Result<usize, ErrBox>, &str)> = vec![
            (Ok(1), "1) jfdlskajfdlksa"),
            (Ok(0), "0) jfdlskajfdlksa"),
            (Ok(22), "22) jfdlskajfdlksa"),
            (Ok(69), "69) 54) jfdlskajfdlksa"),
            (Ok(4000), "4000) jfdlskajfdlksa"),
            (Ok(4000000000), "4000000000) jfdlskajfdlksa"),
            (Err(get_line_parse_err("-12) negawifi")), "-12) negawifi"),
            (Err(get_line_parse_err("jf jfjf")), "jf jfjf"),
        ];

        for (res, line) in test_cases {
            match get_index_of_selected_item(line) {
                Ok(val) => assert_eq![res?, val],
                Err(err) => assert_eq![res.err().unwrap().description(), err.description()],
            }
        }
        Ok(())
    }
}
