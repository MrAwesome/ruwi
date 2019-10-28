use crate::netctl_config_writer::*;
use crate::structs::*;

pub fn send_outputs(
    options: &Options,
    network: &WirelessNetwork,
    encryption_key: &Option<String>,
) -> Vec<Result<OutputResult, OutputError>> {
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
) -> Result<OutputResult, OutputError> {
    // TODO: implement
    // TODO: match output types
    match output_type {
        OutputType::NetctlConfig => netctl_config_write(options, network, encryption_key),
        _ => Err(OutputError::NotImplemented),
    }
}
