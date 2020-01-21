use crate::select_utils::*;
use crate::structs::*;

pub(crate) fn possibly_get_encryption_key(
    options: &WifiOptions,
    selected_network: &AnnotatedWirelessNetwork,
) -> Result<Option<String>, RuwiError> {
    possibly_get_encryption_key_impl(options, selected_network, prompt_for_encryption_key)
}

fn possibly_get_encryption_key_impl<F>(
    options: &WifiOptions,
    selected_network: &AnnotatedWirelessNetwork,
    prompt_func: F,
) -> Result<Option<String>, RuwiError>
where
    F: Fn(&WifiOptions, &str) -> Result<String, RuwiError>,
{
    // Don't bother asking for a password if:
    // * a password was given on the command line
    // * the output type we have doesn't require a password
    // * the network isn't wpa

    let pw = match &options.given_encryption_key {
        Some(pw) => Some(pw.clone()),
        None => match options.connect_via {
            WifiConnectionType::Netctl | WifiConnectionType::NetworkManager => {
                if options.force_ask_password
                    || (!selected_network.known && selected_network.is_encrypted)
                {
                    Some(prompt_func(options, &selected_network.essid)?)
                } else {
                    None
                }
            }
            WifiConnectionType::None | WifiConnectionType::Print => None,
        },
    };

    if options.d() {
        dbg![&pw];
    }

    Ok(pw)
}

fn prompt_for_encryption_key(options: &WifiOptions, network_name: &str) -> Result<String, RuwiError> {
    match options.get_selection_method() {
        SelectionMethod::Dmenu => {
            run_dmenu(options, &format!("Password for {}: ", network_name), &[])
        }
        SelectionMethod::Fzf => run_stdin_prompt_single_line(
            options,
            &format!("Password for {}: ", network_name),
            &[],
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn should_not_run(_opt: &WifiOptions, _nw: &str) -> Result<String, RuwiError> {
        panic!("Should not run.")
    }

    #[test]
    fn test_no_ask_on_open_network() -> Result<(), RuwiError> {
        let options = WifiOptions::default();
        let nw = AnnotatedWirelessNetwork::default();
        let output = possibly_get_encryption_key_impl(&options, &nw, should_not_run)?;
        if let Some(_pw) = output {
            panic!("Got password when none was expected!");
        }
        Ok(())
    }

    #[test]
    fn test_no_ask_on_known_closed_network() -> Result<(), RuwiError> {
        let options = WifiOptions::default();
        let nw = AnnotatedWirelessNetwork {
            is_encrypted: true,
            known: true,
            ..AnnotatedWirelessNetwork::default()
        };
        let output = possibly_get_encryption_key_impl(&options, &nw, should_not_run)?;
        if let Some(_pw) = output {
            panic!("Got password when none was expected!");
        }
        Ok(())
    }

    #[test]
    fn test_ask_on_unknown_closed_network() -> Result<(), RuwiError> {
        let options = WifiOptions::default();
        let nw = AnnotatedWirelessNetwork {
            is_encrypted: true,
            ..AnnotatedWirelessNetwork::default()
        };
        let fake_essid = "FAKE_CLOSURE_VALUE".to_string();
        let output =
            possibly_get_encryption_key_impl(&options, &nw, |_, _| Ok(fake_essid.clone()))?;
        assert_eq![output.unwrap(), fake_essid];
        Ok(())
    }

    #[test]
    fn test_use_given_pw() -> Result<(), RuwiError> {
        let given_essid = "YEETU";
        let options = WifiOptions {
            given_encryption_key: Some("YEETU".into()),
            ..WifiOptions::default()
        };
        let nw = AnnotatedWirelessNetwork::default();
        let output = possibly_get_encryption_key_impl(&options, &nw, should_not_run)?;
        assert_eq![output.unwrap(), given_essid];
        Ok(())
    }

    #[test]
    fn test_force_ask_password() -> Result<(), RuwiError> {
        let options = WifiOptions {
            force_ask_password: true,
            ..WifiOptions::default()
        };

        let nw = AnnotatedWirelessNetwork::default();
        let fake_essid = "FAKE_CLOSURE_VALUE".to_string();
        let output =
            possibly_get_encryption_key_impl(&options, &nw, |_, _| Ok(fake_essid.clone()))?;
        assert_eq![output.unwrap(), fake_essid];
        Ok(())
    }

    #[test]
    fn test_do_not_ask_for_pw_on_print() -> Result<(), RuwiError> {
        let options = WifiOptions {
            connect_via: WifiConnectionType::Print,
            ..WifiOptions::default()
        };

        let nw = AnnotatedWirelessNetwork::default();
        let output = possibly_get_encryption_key_impl(&options, &nw, should_not_run)?;
        if let Some(_pw) = output {
            panic!("Got password when none was expected!");
        }
        Ok(())
    }
}
