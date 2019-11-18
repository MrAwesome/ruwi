use crate::select::*;
use crate::structs::*;
use std::io;

pub(crate) fn get_password(
    options: &Options,
    selected_network: &WirelessNetwork,
) -> io::Result<Option<String>> {
    // Don't bother asking for a password:
    // * a password was given on the command line
    // * the output type we have doesn't require a password
    // * the network isn't wpa
    // TODO(high): unit test this
    let pw = match &options.given_password {
        Some(pw) => Ok(Some(pw.clone())),
        None => match options.output_type {
            OutputType::NetctlConfig | OutputType::NetworkManagerConfig => {
                if selected_network.is_encrypted {
                    Ok(Some(prompt_for_password(options, &selected_network.essid)?))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        },
    };

    options.dbg(&pw);

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
