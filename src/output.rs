use crate::netctl_config_writer::*;
use crate::structs::*;
use std::io;

// For multiple outputs:
//pub(crate) fn send_outputs(
//    options: &Options,
//    network: &WirelessNetwork,
//    encryption_key: &Option<String>,
//) -> Vec<io::Result<OutputResult>> {
//    options
//        .output_types
//        .iter()
//        .map(|opt| send_output(options, opt, network, encryption_key))
//        .collect()
//}

pub(crate) fn send_output(
    options: &Options,
    network: &WirelessNetwork,
    encryption_key: &Option<String>,
) -> io::Result<OutputResult> {
    let res = match &options.output_type {
        OutputType::NetctlConfig => netctl_config_write(options, network, encryption_key),
        OutputType::None => Ok(OutputResult {
            output_type: OutputType::None,
            output_output: None,
        }),
        x => Err(nie(x)),
    };

    options.dbg((&options.output_type, &res));

    res
}
