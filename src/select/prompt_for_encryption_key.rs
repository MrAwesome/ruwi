use super::external_selection_programs::*;
use crate::enums::SelectionMethod;
use crate::errors::*;
use crate::options::traits::*;

pub(crate) fn prompt_for_encryption_key<O>(
    options: &O,
    network_name: &str,
) -> Result<String, RuwiError>
where
    O: Global,
{
    match options.get_selection_method() {
        SelectionMethod::Dmenu => {
            run_dmenu(options, &format!("Password for {}: ", network_name), &[])
        }
        SelectionMethod::Fzf | SelectionMethod::NoCurses => {
            run_stdin_prompt_single_line(options, &format!("Password for {}: ", network_name), &[])
        }
    }
}
