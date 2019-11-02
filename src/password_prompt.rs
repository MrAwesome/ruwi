use crate::select::*;
use crate::structs::*;
use std::io;

pub(crate) fn get_password(
    options: &Options,
    selected_network: &Option<WirelessNetwork>,
) -> io::Result<Option<String>> {
    // If we didn't select a network, the output type we have doesn't require a password, or the
    // network isn't wpa, don't bother asking for a password.
    let pw = match selected_network {
        Some(nw) => match options.output_type {
            OutputType::NetctlConfig | OutputType::NetworkManagerConfig => {
                if nw.wpa {
                    Ok(Some(prompt_for_password(options, &nw.essid)?))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        },
        None => Ok(None),
    };

    if options.debug {
        dbg!(&pw);
    }

    pw
}

pub(crate) fn prompt_for_password(options: &Options, network_name: &str) -> io::Result<String> {
    match &options.selection_method {
        SelectionMethod::Dmenu => {
            run_dmenu(options, &format!("Password for {}: ", network_name), &[])
        }
        SelectionMethod::Fzf => {
            run_stdin_prompt_single_line(options, &format!("Password for {}: ", network_name), &[])
        }
    }
}
