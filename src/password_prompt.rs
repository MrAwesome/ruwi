use crate::select::*;
use crate::structs::*;
use std::io;

pub fn get_password(
    options: &Options,
    selected_network: &WirelessNetwork,
) -> io::Result<Option<String>> {
    if selected_network.wpa {
        let pw = prompt_for_password(&options, &selected_network.essid)?;
        Ok(Some(pw))
    } else {
        Ok(None)
    }
}

pub fn prompt_for_password(options: &Options, network_name: &String) -> io::Result<String> {
    match &options.selection_method {
        SelectionMethod::Dmenu => {
            run_dmenu(options, &format!("Password for {}:", network_name), &vec![])
        }
        x @ SelectionMethod::Fzf => Err(nie(x)),
    }
}
