use super::utils::*;
use super::WIRED_CONNECT_TOKEN;

use crate::enums::*;
use crate::options::wired::connect::WiredConnectOptions;
use crate::options::command::*;
use crate::options::wired::*;
use crate::errors::*;
use crate::options::*;
use crate::strum_utils::*;

use clap::ArgMatches;

pub(super) fn get_wired_cmd(
    globals: GlobalOptions,
    maybe_wired_matcher: Option<&ArgMatches>,
) -> Result<RuwiWiredCommand, RuwiError> {
    if let Some(wired_matcher) = maybe_wired_matcher {
        let wired_opts = get_wired_opts_impl(globals, wired_matcher)?;
        let (subcommand_name, subcommand_matcher) = wired_matcher.subcommand();

        let cmd = if subcommand_name == "" || subcommand_name == WIRED_CONNECT_TOKEN {
            RuwiWiredCommand::Connect(get_wired_connect_opts(wired_opts, subcommand_matcher)?)
        } else {
            handle_cmdline_parsing_error(subcommand_name, subcommand_matcher)?
        };

        Ok(cmd)
    } else {
        get_default_wired_command(globals)
    }
}

fn get_default_wired_command(globals: GlobalOptions) -> Result<RuwiWiredCommand, RuwiError> {
    Ok(RuwiWiredCommand::Connect(
        WiredConnectOptions::builder()
            .wired(
                WiredOptions::builder()
                    .globals(globals)
                    .build(),
            )
            .build(),
    ))
}

fn get_wired_connect_opts(
    wired_opts: WiredOptions,
    maybe_connect_matcher: Option<&ArgMatches>,
) -> Result<WiredConnectOptions, RuwiError> {
    let connect_builder = WiredConnectOptions::builder().wired(wired_opts);
    let connect_opts = if let Some(connect_matcher) = maybe_connect_matcher {
        let connect_via = get_val_as_enum::<RawInterfaceConnectionType>(&connect_matcher, "connect_via");

        connect_builder
            .connect_via(connect_via)
            .build()
    } else {
        connect_builder.build()
    };
    Ok(connect_opts)
}

fn get_wired_opts_impl(
    globals: GlobalOptions,
    wired_matcher: &ArgMatches,
) -> Result<WiredOptions, RuwiError> {
    let given_interface_name = wired_matcher.value_of("interface").map(String::from);

    let wired_opts = WiredOptions::builder()
        .globals(globals)
        .given_interface_name(given_interface_name)
        .build();

    Ok(wired_opts)
}
