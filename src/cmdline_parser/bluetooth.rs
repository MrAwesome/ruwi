use super::utils::handle_cmdline_parsing_error;
use super::BLUETOOTH_CONNECT_TOKEN;

use crate::options::command::RuwiBluetoothCommand;
use crate::options::bluetooth::connect::BluetoothConnectOptions;
use crate::options::bluetooth::BluetoothOptions;
use crate::options::GlobalOptions;
use crate::prelude::*;

use clap::ArgMatches;

pub(super) fn get_bluetooth_cmd(
    globals: GlobalOptions,
    maybe_bluetooth_matcher: Option<&ArgMatches>,
) -> Result<RuwiBluetoothCommand, RuwiError> {
    if let Some(bluetooth_matcher) = maybe_bluetooth_matcher {
        let bluetooth_opts = get_bluetooth_opts_impl(globals, bluetooth_matcher)?;
        let (subcommand_name, subcommand_matcher) = bluetooth_matcher.subcommand();

        let cmd = if subcommand_name == "" || subcommand_name == BLUETOOTH_CONNECT_TOKEN {
            RuwiBluetoothCommand::Connect(get_bluetooth_connect_opts(bluetooth_opts, subcommand_matcher)?)
        } else {
            handle_cmdline_parsing_error(subcommand_name, subcommand_matcher)?
        };

        Ok(cmd)
    } else {
        get_default_bluetooth_command(globals)
    }
}

fn get_default_bluetooth_command(globals: GlobalOptions) -> Result<RuwiBluetoothCommand, RuwiError> {
    Ok(RuwiBluetoothCommand::Connect(
        BluetoothConnectOptions::builder()
            .bluetooth(BluetoothOptions::builder().globals(globals).build())
            .build(),
    ))
}

fn get_bluetooth_connect_opts(
    bluetooth_opts: BluetoothOptions,
    maybe_connect_matcher: Option<&ArgMatches>,
) -> Result<BluetoothConnectOptions, RuwiError> {
    let connect_builder = BluetoothConnectOptions::builder();
    let connect_opts = if let Some(_connect_matcher) = maybe_connect_matcher {
        // TODO: more specific options here
        connect_builder
            .bluetooth(bluetooth_opts)
            .build()
    } else {
        connect_builder.bluetooth(bluetooth_opts).build()
    };
    Ok(connect_opts)
}

fn get_bluetooth_opts_impl(
    globals: GlobalOptions,
    _bluetooth_matcher: &ArgMatches,
) -> Result<BluetoothOptions, RuwiError> {
    let bluetooth_opts = BluetoothOptions::builder()
        .globals(globals)
        .build();

    Ok(bluetooth_opts)
}
