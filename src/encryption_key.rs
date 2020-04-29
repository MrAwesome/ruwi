use crate::prelude::*;
use crate::select::prompt_for_encryption_key::prompt_for_encryption_key;

pub(crate) fn possibly_get_encryption_key<O>(
    options: &O,
    selected_network: &AnnotatedWirelessNetwork,
) -> Result<Option<String>, RuwiError>
where
    O: Global + Wifi + WifiConnect,
{
    possibly_get_encryption_key_impl(options, selected_network, prompt_for_encryption_key)
}

fn possibly_get_encryption_key_impl<O, F>(
    options: &O,
    selected_network: &AnnotatedWirelessNetwork,
    prompt_func: F,
) -> Result<Option<String>, RuwiError>
where
    O: Global + Wifi + WifiConnect,
    F: Fn(&O, &str) -> Result<String, RuwiError>,
{
    // Don't bother asking for a password if:
    // * a password was given on the command line
    // * the output type we have doesn't require a password
    // * the network isn't wpa

    let pw = match &options.get_given_encryption_key() {
        Some(pw) => Some(pw.clone()),
        None => match options.get_connect_via() {
            WifiConnectionType::Netctl | WifiConnectionType::Nmcli => {
                if options.get_force_ask_password()
                    || (!selected_network.is_known() && selected_network.is_encrypted())
                {
                    Some(prompt_func(options, selected_network.get_public_name().as_ref())?)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::wifi::connect::WifiConnectOptions;
    use crate::options::wifi::WifiOptions;

    fn should_not_run(_opt: &WifiConnectOptions, _nw: &str) -> Result<String, RuwiError> {
        panic!("Should not run.")
    }

    #[test]
    fn test_no_ask_on_open_network() -> Result<(), RuwiError> {
        let options = WifiConnectOptions::default();
        let nw = AnnotatedWirelessNetwork::from_essid_only("made_up");
        let output = possibly_get_encryption_key_impl(&options, &nw, should_not_run)?;
        if let Some(_pw) = output {
            panic!("Got password when none was expected!");
        }
        Ok(())
    }

    #[test]
    fn test_no_ask_on_known_closed_network() -> Result<(), RuwiError> {
        let options = WifiConnectOptions::default();
        let nw = AnnotatedWirelessNetwork::builder()
            .essid("MADE_UP")
            .service_identifier(NetworkServiceIdentifier::netctl_nw("I_AM_KNOWN"))
            .is_encrypted(true)
            .build();
        let output = possibly_get_encryption_key_impl(&options, &nw, should_not_run)?;
        if let Some(_pw) = output {
            panic!("Got password when none was expected!");
        }
        Ok(())
    }

    #[test]
    fn test_ask_on_unknown_closed_network() -> Result<(), RuwiError> {
        let options = WifiConnectOptions::default();
        let fake_essid = "FAKE_CLOSURE_VALUE";
        let nw = AnnotatedWirelessNetwork::builder()
            .essid(fake_essid)
            .service_identifier(None)
            .is_encrypted(true)
            .build();
        let output =
            possibly_get_encryption_key_impl(&options, &nw, |_, _| Ok(fake_essid.to_string()))?;
        assert_eq![output.unwrap(), fake_essid];
        Ok(())
    }

    #[test]
    fn test_use_given_pw() -> Result<(), RuwiError> {
        let given_essid = "YEETU";
        let options = WifiConnectOptions::builder()
            .wifi(WifiOptions::default())
            .given_encryption_key(Some("YEETU".into()))
            .build();
        let nw = AnnotatedWirelessNetwork::from_essid_only("FAKE");
        let output = possibly_get_encryption_key_impl(&options, &nw, should_not_run)?;
        assert_eq![output.unwrap(), given_essid];
        Ok(())
    }

    #[test]
    fn test_force_ask_password() -> Result<(), RuwiError> {
        let options = WifiConnectOptions::builder()
            .wifi(WifiOptions::default())
            .force_ask_password(true)
            .build();

        let nw = AnnotatedWirelessNetwork::from_essid_only("FAKE");
        let fake_essid = "FAKE_CLOSURE_VALUE".to_string();
        let output =
            possibly_get_encryption_key_impl(&options, &nw, |_, _| Ok(fake_essid.clone()))?;
        assert_eq![output.unwrap(), fake_essid];
        Ok(())
    }

    #[test]
    fn test_do_not_ask_for_pw_on_print() -> Result<(), RuwiError> {
        let options = WifiConnectOptions::builder()
            .wifi(WifiOptions::default())
            .connect_via(WifiConnectionType::Print)
            .build();

        let nw = AnnotatedWirelessNetwork::from_essid_only("FAKE");
        let output = possibly_get_encryption_key_impl(&options, &nw, should_not_run)?;
        if let Some(_pw) = output {
            panic!("Got password when none was expected!");
        }
        Ok(())
    }
}
