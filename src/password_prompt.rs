use crate::select::*;
use crate::structs::*;

pub fn get_password(
    options: &Options,
    selected_network: &WirelessNetwork,
) -> Result<Option<String>, SelectionError> {
    if selected_network.wpa {
        let pw = prompt_for_password(&options, &selected_network.essid)?;
        Ok(Some(pw))
    } else {
        Ok(None)
    }
}

pub fn prompt_for_password(
    options: &Options,
    network_name: &String,
) -> Result<String, SelectionError> {
    match &options.selection_method {
        SelectionMethod::Dmenu => {
            run_dmenu(options, &format!("Password for {}:", network_name), &vec![])
        }
        SelectionMethod::Fzf => Err(SelectionError::NotImplemented),
    }
}
