use crate::errors::*;
use crate::rerr;
use clap::ArgMatches;

pub(super) fn handle_cmdline_parsing_error<T>(
    invalid_subc_name: &str,
    maybe_sub_matcher: Option<&ArgMatches<'_>>,
) -> Result<T, RuwiError> {
    Err(rerr!(
        RuwiErrorKind::InvalidSubcommand,
        format!("[ERR] Unknown command name: {}", invalid_subc_name)
    ))
}
