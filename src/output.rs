use crate::netctl_config_writer::*;
use crate::structs::*;
use std::io;

pub fn send_outputs(
    options: &Options,
    network: &WirelessNetwork,
    encryption_key: &Option<String>,
) -> Vec<io::Result<OutputResult>> {
    options
        .output_types
        .iter()
        .map(|opt| send_output(options, opt, network, encryption_key))
        .collect()
}

fn send_output(
    options: &Options,
    output_type: &OutputType,
    network: &WirelessNetwork,
    encryption_key: &Option<String>,
) -> io::Result<OutputResult> {
    // TODO: implement
    // TODO: match output types
    let res = match output_type {
        OutputType::NetctlConfig => netctl_config_write(options, network, encryption_key),
        x => Err(nie(x)),
    };

    if options.debug {
        dbg!(&output_type, &res);
    }

    res
}
