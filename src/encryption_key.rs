use crate::select::*;
use crate::structs::*;

pub(crate) fn possibly_get_encryption_key(
    options: &Options,
    selected_network: &AnnotatedWirelessNetwork,
) -> Result<Option<String>, RuwiError> {
    // Don't bother asking for a password:
    // * a password was given on the command line
    // * the output type we have doesn't require a password
    // * the network isn't wpa
    // TODO(high): unit test this
    let pw = match &options.given_encryption_key {
        Some(pw) => Some(pw.clone()),
        None => match options.connect_via {
            ConnectionType::Netctl | ConnectionType::NetworkManager => {
                if !selected_network.known && selected_network.is_encrypted {
                    Some(prompt_for_encryption_key(options, &selected_network.essid)?)
                } else {
                    None
                }
            }
            ConnectionType::None => None,
        },
    };

    if options.debug {
        dbg![&pw];
    }

    Ok(pw)
}

pub(crate) fn prompt_for_encryption_key(
    options: &Options,
    network_name: &str,
) -> Result<String, RuwiError> {
    match &options.selection_method {
        SelectionMethod::Dmenu => {
            run_dmenu(options, &format!("Password for {}: ", network_name), vec![])
        }
        SelectionMethod::Fzf => run_stdin_prompt_single_line(
            options,
            &format!("Password for {}: ", network_name),
            vec![],
        ),
    }
}
