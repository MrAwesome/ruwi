use super::utils::*;
use super::WIRED_CONNECT_TOKEN;

use crate::interface_management::ip_interfaces::WiredIPInterface;

use crate::enums::*;
use crate::options::wired::connect::WiredConnectOptions;
use crate::options::command::*;
use crate::options::wired::*;
use crate::options::interfaces::*;
use crate::errors::*;
use crate::options::*;
use crate::strum_utils::*;

use clap::ArgMatches;
use std::fmt::Debug;

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
    let interface = WiredIPInterface::find_first(&globals)?;
    Ok(RuwiWiredCommand::Connect(
        WiredConnectOptions::builder()
            .wired(
                WiredOptions::builder()
                    .globals(globals)
                    .interface(interface)
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
        let connect_via = get_val_as_enum::<WiredConnectionType>(&connect_matcher, "connect_via");

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
    sub_m: &ArgMatches,
) -> Result<WiredOptions, RuwiError> {
    let interface = get_wired_interface(sub_m, &globals)?;

    let wired_opts = WiredOptions::builder()
        .globals(globals)
        .interface(interface)
        .build();

    Ok(wired_opts)
}

fn get_wired_interface<O>(m: &ArgMatches, opts: &O) -> Result<WiredIPInterface, RuwiError>
where
    O: Global + Debug,
{
    Ok(match m.value_of("interface") {
        Some(given_ifname) => WiredIPInterface::new(given_ifname),
        None => WiredIPInterface::find_first(opts)?,
    })
}

