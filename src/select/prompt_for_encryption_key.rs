use crate::errors::*;
use crate::options::interfaces::*;
use crate::structs::SelectionMethod;
use super::external_selection_programs::*;

pub(crate) fn prompt_for_encryption_key<O>(options: &O, network_name: &str) -> Result<String, RuwiError>
where
    O: Global,
{
    match options.get_selection_method() {
        SelectionMethod::Dmenu => {
            run_dmenu(options, &format!("Password for {}: ", network_name), &[])
        }
        SelectionMethod::Fzf => {
            run_stdin_prompt_single_line(options, &format!("Password for {}: ", network_name), &[])
        }
    }
}
