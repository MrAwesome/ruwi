use crate::netctl_config_writer::*;
use crate::structs::*;

pub fn send_outputs(
    options: Options,
    network: WirelessNetwork,
    encryption_key: Option<String>,
) -> Vec<Result<OutputResult, OutputError>> {
    options
        .output_types
        .iter()
        .map(|opt| {
            send_output(
                options.clone(),
                opt.clone(),
                network.clone(),
                encryption_key.clone(),
            )
        })
        .collect()
}

fn send_output(
    options: Options,
    output_type: OutputType,
    network: WirelessNetwork,
    encryption_key: Option<String>,
) -> Result<OutputResult, OutputError> {
    // TODO: implement
    // TODO: match output types
    let res = netctl_config_write(options.clone(), network.clone(), encryption_key.clone());
    dbg![res.clone()];
    res
}
