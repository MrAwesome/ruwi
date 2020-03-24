use crate::enums::*;
use crate::errors::*;
use crate::options::interfaces::*;
use crate::structs::*;

pub(crate) fn parse_nmcli_scan<O>(
    _options: &O,
    output: &str,
    scan_type: ScanType,
) -> Result<ParseResult, RuwiError>
where
    O: Global,
{
    let mut seen_networks = vec![];
    let mut line_parse_errors = vec![];
    for line in output.trim().lines() {
        let res = get_network_from_nmcli_line(line);
        match res {
            Ok(nw) => seen_networks.push(nw),
            Err(err) => line_parse_errors.push((line.into(), err)),
        };
    }
    Ok(ParseResult {
        scan_type,
        seen_networks,
        line_parse_errors,
    })
}

fn get_network_from_nmcli_line(line: &str) -> Result<WirelessNetwork, IndividualParseError> {
    let mut tokens = line.splitn(3, ':');

    let enc_txt = tokens
        .next()
        .ok_or(IndividualParseError::MissingNmcliSeparator)?;
    let signal_strength_txt = tokens
        .next()
        .ok_or(IndividualParseError::MissingNmcliSeparator)?;
    let essid_txt = tokens
        .next()
        .ok_or(IndividualParseError::MissingNmcliSeparator)?;

    let is_encrypted = !enc_txt.is_empty();
    let signal_strength = signal_strength_txt.parse::<i32>().ok();
    let essid = essid_txt.to_string();

    Ok(WirelessNetwork::builder()
        .essid(essid)
        .is_encrypted(is_encrypted)
        .signal_strength(signal_strength)
    .build())
}
