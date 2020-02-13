use crate::strum_utils::possible_vals;
use std::fmt::Debug;

use strum_macros::{AsStaticStr, Display, EnumIter, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Display, AsStaticStr)]
#[strum(serialize_all = "snake_case")]
pub(super) enum SelectionOption {
    Refresh,
}

pub(super) fn get_possible_selection_options_as_strings() -> Vec<String> {
    possible_vals::<SelectionOption, _>()
        .iter()
        .map(|&x| x.into())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_refresh_is_present() {
        assert![get_possible_selection_options_as_strings().contains(&"refresh".to_string())];
    }
}
