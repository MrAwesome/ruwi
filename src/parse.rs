use crate::structs::*;
use regex::Regex;
use std::io;
use unescape::unescape;

pub(crate) fn parse_result(options: &Options, scan_result: &ScanResult) -> io::Result<ParseResult> {
    // TODO: if scan type isn't specified, and parsing or scanning fails, try another scan type
    let res = match &scan_result.scan_type {
        ScanType::WpaCli => parse_wpa_cli_scan(options, &scan_result.output),
        ScanType::IW => parse_iw_scan(options, &scan_result.output),
        x @ ScanType::IWList => Err(nie(x)),
    };

    if options.debug {
        dbg!(&res);
    }

    res
}

fn parse_iw_scan(options: &Options, output: &str) -> io::Result<ParseResult> {
    let network_chunks = break_iw_output_into_chunks_per_network(options, output)?;
    let mut seen_networks = vec![];
    let mut line_parse_errors = vec![];
    for chunk in network_chunks {
        let res = parse_iw_chunk_into_network(&chunk);
        match res {
            Ok(nw) => seen_networks.push(nw),
            Err(err) => line_parse_errors.push((chunk.join("\n"), err)),
        };
    }
    Ok(ParseResult {
        scan_type: ScanType::IW,
        seen_networks,
        line_parse_errors,
    })
}

fn break_iw_output_into_chunks_per_network(
    options: &Options,
    output: &str,
) -> io::Result<Vec<Vec<String>>> {
    let mut lines = output.trim().lines().map(|line| line.trim().to_string());
    let mut iw_network_line_groups = vec![];

    let bss_re = get_iw_bss_regex();

    let mut network = vec![];

    if let Some(line) = lines.next() {
        if bss_re.is_match(&line) {
            network.push(line);
        } else {
            Err(err_iw_malformed_output(options))?
        }
    } else {
        Err(err_iw_no_networks_seen(options))?
    }

    loop {
        //while let Some(line) = lines.next() {
        match lines.next() {
            Some(line) => {
                if bss_re.is_match(&line) {
                    iw_network_line_groups.push(network);
                    network = vec![];
                }

                network.push(line);
            }
            None => {
                iw_network_line_groups.push(network);
                break;
            }
        }
    }
    Ok(iw_network_line_groups)
}

fn parse_iw_chunk_into_network(
    chunk: &Vec<String>,
) -> Result<WirelessNetwork, IndividualParseError> {
    let essid = unescape(
        chunk
            .iter()
            .filter(|line| line.starts_with("SSID: "))
            .map(|line| line.trim_start_matches("SSID: "))
            .next()
            .ok_or(IndividualParseError::MissingIWSSIDField)?,
    )
    .ok_or(IndividualParseError::FailedToUnescapeSSIDField)?;

    let is_encrypted = chunk
        .iter()
        .filter(|line| line.starts_with("capability:"))
        .next()
        .ok_or(IndividualParseError::MissingIWCapabilityField)?
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .contains(&"Privacy");

    let bssid = chunk
        .first()
        .ok_or(IndividualParseError::ZeroLengthIWChunk)?
        .trim_start_matches("BSS ")
        .split("(")
        .map(|x| x.to_string())
        .next();

    let signal_strength = chunk
        .iter()
        .filter(|line| line.starts_with("signal: "))
        .filter_map(|line| {
            line.trim_start_matches("signal: ")
                .trim_end_matches(" dBm")
                .split(".")
                .next()?
                .parse::<i32>()
                .ok()
        })
        .next();

    let channel_utilisation = None; // TODO: implement if needed

    Ok(WirelessNetwork {
        essid,
        is_encrypted,
        bssid,
        signal_strength,
        channel_utilisation,
    })
}

fn get_iw_bss_regex() -> Regex {
    Regex::new(r"^BSS ((\w\w:){5}\w\w)").expect("Failure creating regex for iw parsing...")
}

// TODO: put the actual command run here
// TODO: if dump, suggest scan
fn err_iw_malformed_output(options: &Options) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!(
            "Malformed output returned by `sudo iw {} scan dump`. Try running it manually.",
            options.interface
        ),
    )
}

fn err_iw_no_networks_seen(options: &Options) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("No networks seen by `sudo iw {} scan dump`. Are you near wireless networks? Try running `sudo iw {} scan`.", options.interface, options.interface))
}

fn parse_wpa_cli_scan(_options: &Options, output: &str) -> io::Result<ParseResult> {
    let mut lines = output.trim().lines().map(|x| x.to_string());
    let mut networks = vec![];
    let mut line_parse_errors = vec![];

    let _header1 = lines.next().ok_or_else(missing_wpa_cli_header)?;
    let _header2 = lines.next().ok_or_else(missing_wpa_cli_header)?;

    for line in lines {
        let res = parse_wpa_line_into_network(line.to_string());
        match res {
            Ok(nw) => networks.push(nw),
            Err(err) => line_parse_errors.push((line, err)),
        };
    }

    if networks.is_empty() {
        // TODO(wishlist): if individual parse errors happened, print them?
        //                 maybe only in debug mode?
        Err(err_wpa_cli_no_networks_seen())
    } else {
        Ok(ParseResult {
            scan_type: ScanType::WpaCli,
            seen_networks: networks,
            line_parse_errors,
        })
    }
}

fn err_wpa_cli_no_networks_seen() -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("No networks seen by `sudo wpa_cli scan_results`. Are you near wireless networks? Try running `sudo wpa_cli scan`."))
}

fn missing_wpa_cli_header() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        "`wpa_cli scan_results` header malformed or missing",
    )
}

fn parse_wpa_line_into_network(line: String) -> Result<WirelessNetwork, IndividualParseError> {
    let mut fields = line.split_ascii_whitespace();

    let fieldmiss = IndividualParseError::MissingWpaCliResultField;
    let bssid = fields.next().ok_or(fieldmiss)?;
    let _freq = fields.next().ok_or(fieldmiss)?;
    let signal_level = fields.next().ok_or(fieldmiss)?;
    let flags = fields.next().ok_or(fieldmiss)?;

    // NOTE: Broken for SSIDs with multiple sequential spaces, or trailing whitespace
    let essid = fields
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    let is_encrypted = flags.contains("WPA");
    let signal_strength = signal_level
        .parse::<i32>()
        .or(Err(IndividualParseError::FailedToParseSignalLevel))?;

    Ok(WirelessNetwork {
        essid,
        is_encrypted,
        bssid: Some(bssid.to_string()),
        signal_strength: Some(signal_strength),
        channel_utilisation: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    pub static IW_ONE_NETWORK: &'static str = include_str!("samples/iw_one_network.txt");
    pub static IW_TWO_DIFFERENT_NETWORKS: &'static str =
        include_str!("samples/iw_two_different_networks.txt");
    pub static WPA_CLI_NO_NETWORKS: &'static str = include_str!("samples/wpa_cli_no_networks.txt");
    pub static WPA_CLI_TWO_DIFFERENT_NETWORKS: &'static str =
        include_str!("samples/wpa_cli_two_different_networks.txt");
    pub static WPA_CLI_SEVEN_NETWORKS_TWO_DUPLICATE_TWO_EMPTY: &'static str =
        include_str!("samples/wpa_cli_seven_networks_two_duplicates_two_empty.txt");
    pub static WPA_CLI_TWO_LINES_MISSING_INFO: &'static str =
        include_str!("samples/wpa_cli_two_lines_missing_info.txt");
    pub static WPA_CLI_TWO_NETWORKS_ONE_WITH_SIGNAL_LEVEL_PARSE_ERROR: &'static str =
        include_str!("samples/wpa_cli_two_networks_one_with_signal_level_parse_error.txt");
    pub static BROKEN_INPUT_TWO_WORDS: &'static str =
        include_str!("samples/broken_input_two_words.txt");

    fn get_iw_basic_options() -> Options {
        let mut opts = Options::default();
        opts.scan_type = ScanType::IW;
        opts
    }

    fn get_wpa_cli_basic_options() -> Options {
        let mut opts = Options::default();
        opts.scan_type = ScanType::WpaCli;
        opts
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    enum NetworkTextType {
        IWOneNetwork,
        IWTwoDifferentNetworks,
        WpaCliNoNetworks,
        WpaCliTwoDifferentNetworks,
        WpaCliSevenNetworksTwoDuplicateTwoEmpty,
        WpaCliTwoLinesMissingInfo,
        WpaCliTwoNetworksOneWithSignalLevelParseError,
        BrokenInputTwoWords,
    }

    fn get_text(text_type: NetworkTextType) -> String {
        match text_type {
            NetworkTextType::IWOneNetwork => self::IW_ONE_NETWORK.to_string(),
            NetworkTextType::IWTwoDifferentNetworks => self::IW_TWO_DIFFERENT_NETWORKS.to_string(),

            NetworkTextType::WpaCliTwoDifferentNetworks => {
                self::WPA_CLI_TWO_DIFFERENT_NETWORKS.to_string()
            }
            NetworkTextType::WpaCliNoNetworks => self::WPA_CLI_NO_NETWORKS.to_string(),
            NetworkTextType::WpaCliSevenNetworksTwoDuplicateTwoEmpty => {
                self::WPA_CLI_SEVEN_NETWORKS_TWO_DUPLICATE_TWO_EMPTY.to_string()
            }
            NetworkTextType::WpaCliTwoLinesMissingInfo => {
                self::WPA_CLI_TWO_LINES_MISSING_INFO.to_string()
            }
            NetworkTextType::WpaCliTwoNetworksOneWithSignalLevelParseError => {
                self::WPA_CLI_TWO_NETWORKS_ONE_WITH_SIGNAL_LEVEL_PARSE_ERROR.to_string()
            }
            NetworkTextType::BrokenInputTwoWords => self::BROKEN_INPUT_TWO_WORDS.to_string(),
        }
    }

    fn get_expected_result(text_type: NetworkTextType) -> io::Result<ParseResult> {
        match text_type {
            NetworkTextType::IWOneNetwork => Ok(ParseResult {
                scan_type: ScanType::IW,
                seen_networks: vec![WirelessNetwork {
                    essid: "Pee Pee Poo Poo Man".to_string(),
                    is_encrypted: true,
                    bssid: Some("32:ac:a3:7b:ab:0b".to_string()),
                    signal_strength: Some(-38),
                    channel_utilisation: None,
                }],
                line_parse_errors: vec![],
            }),
            NetworkTextType::IWTwoDifferentNetworks => Ok(ParseResult {
                scan_type: ScanType::IW,
                seen_networks: vec![
                    WirelessNetwork {
                        essid: "Valparaiso_Guest_House 1".to_string(),
                        is_encrypted: true,
                        bssid: Some("f4:28:53:fe:a5:d0".to_string()),
                        signal_strength: Some(-65),
                        channel_utilisation: None,
                    },
                    WirelessNetwork {
                        essid: "Valparaiso_Guest_House 2".to_string(),
                        is_encrypted: true,
                        bssid: Some("68:72:51:68:73:da".to_string()),
                        signal_strength: Some(-46),
                        channel_utilisation: None,
                    },
                ],
                line_parse_errors: vec![],
            }),
            NetworkTextType::WpaCliTwoDifferentNetworks => Ok(ParseResult {
                scan_type: ScanType::WpaCli,
                seen_networks: vec![
                    WirelessNetwork {
                        essid: "Valparaiso_Guest_House 1".to_string(),
                        is_encrypted: true,
                        bssid: Some("f4:28:53:fe:a5:d0".to_string()),
                        signal_strength: Some(-66),
                        channel_utilisation: None,
                    },
                    WirelessNetwork {
                        essid: "Valparaiso_Guest_House 2".to_string(),
                        is_encrypted: true,
                        bssid: Some("68:72:51:68:73:da".to_string()),
                        signal_strength: Some(-47),
                        channel_utilisation: None,
                    },
                ],
                line_parse_errors: vec![],
            }),
            NetworkTextType::WpaCliNoNetworks => Err(err_wpa_cli_no_networks_seen()),

            NetworkTextType::WpaCliSevenNetworksTwoDuplicateTwoEmpty => Ok(ParseResult {
                scan_type: ScanType::WpaCli,
                seen_networks: vec![
                    WirelessNetwork {
                        essid: "Nima Lodge".to_string(),
                        is_encrypted: true,
                        bssid: Some("78:8a:20:e3:9d:62".to_string()),
                        signal_strength: Some(-41),
                        channel_utilisation: None,
                    },
                    WirelessNetwork {
                        essid: "DIRECT-AF-HP DeskJet 3830 series".to_string(),
                        is_encrypted: true,
                        bssid: Some("fc:3f:db:a1:5e:b0".to_string()),
                        signal_strength: Some(-68),
                        channel_utilisation: None,
                    },
                    WirelessNetwork {
                        essid: "Nima Lodge".to_string(),
                        is_encrypted: true,
                        bssid: Some("fc:ec:da:69:e0:3e".to_string()),
                        signal_strength: Some(-85),
                        channel_utilisation: None,
                    },
                    WirelessNetwork {
                        essid: "".to_string(),
                        is_encrypted: true,
                        bssid: Some("fe:ec:da:69:e0:3e".to_string()),
                        signal_strength: Some(-85),
                        channel_utilisation: None,
                    },
                    WirelessNetwork {
                        essid: "WISPerNET-George-Sentraal1".to_string(),
                        is_encrypted: false,
                        bssid: Some("ba:69:f4:1f:2d:15".to_string()),
                        signal_strength: Some(-89),
                        channel_utilisation: None,
                    },
                    WirelessNetwork {
                        essid: "WISPerNET-Bosplaas-SW-802.11".to_string(),
                        is_encrypted: false,
                        bssid: Some("b8:69:f4:1f:2d:15".to_string()),
                        signal_strength: Some(-89),
                        channel_utilisation: None,
                    },
                    WirelessNetwork {
                        essid: "".to_string(),
                        is_encrypted: true,
                        bssid: Some("7a:8a:20:e3:9d:62".to_string()),
                        signal_strength: Some(-39),
                        channel_utilisation: None,
                    },
                ],
                line_parse_errors: vec![],
            }),
            NetworkTextType::WpaCliTwoLinesMissingInfo => Ok(ParseResult {
                scan_type: ScanType::WpaCli,
                seen_networks: vec![WirelessNetwork {
                    essid: "Valparaiso_Guest_House 1".to_string(),
                    is_encrypted: true,
                    bssid: Some("f4:28:53:fe:a5:d0".to_string()),
                    signal_strength: Some(-66),
                    channel_utilisation: None,
                }],
                line_parse_errors: vec![
                    (
                        "f4:28:53:fe:a5:d0\t2437\t-66".to_string(),
                        IndividualParseError::MissingWpaCliResultField,
                    ),
                    (
                        "68:72:51:68:73:da\t2457".to_string(),
                        IndividualParseError::MissingWpaCliResultField,
                    ),
                ],
            }),
            NetworkTextType::WpaCliTwoNetworksOneWithSignalLevelParseError => Ok(ParseResult {
                scan_type: ScanType::WpaCli,
                seen_networks: vec![WirelessNetwork {
                    essid: "Valparaiso_Guest_House 1".to_string(),
                    is_encrypted: true,
                    bssid: Some("f4:28:53:fe:a5:d0".to_string()),
                    signal_strength: Some(-66),
                    channel_utilisation: None,
                }],
                line_parse_errors: vec![(
                    "68:72:51:68:73:da\t2457\t-xx\t[WPA2-PSK-CCMP][ESS]\tValparaiso_Guest_House 2"
                        .to_string(),
                    IndividualParseError::FailedToParseSignalLevel,
                )],
            }),
            NetworkTextType::BrokenInputTwoWords => Err(missing_wpa_cli_header()),
        }
    }

    fn compare_parsed_result_to_expected_result(text_type: NetworkTextType, options: Options) {
        let contents = get_text(text_type);
        let full_expected_result = get_expected_result(text_type);

        let scan_result = ScanResult {
            scan_type: options.scan_type.clone(),
            output: contents,
        };

        let full_parse_result = parse_result(&options, &scan_result);

        match (&full_parse_result, &full_expected_result) {
            (Ok(parse_result), Ok(expected_result)) => {
                assert_eq![parse_result, expected_result];
            }
            (Err(parse_error), Err(expected_error)) => {
                let parse_desc = parse_error.to_string();
                let expct_desc = expected_error.to_string();
                assert_eq![parse_desc, expct_desc];
                let parse_kind = parse_error.kind();
                let expct_kind = expected_error.kind();
                assert_eq![parse_kind, expct_kind];
            }
            (_, _) => {
                println!("Full parse result: {:?}", full_parse_result);
                println!("Expt parse result: {:?}", full_expected_result);
                assert![false];
            }
        }
    }

    #[test]
    fn test_iw_one_network() {
        let text_type = NetworkTextType::IWOneNetwork;
        let options = get_iw_basic_options();
        compare_parsed_result_to_expected_result(text_type, options);
    }

    #[test]
    fn test_iw_two_different_networks() {
        let text_type = NetworkTextType::IWTwoDifferentNetworks;
        let options = get_iw_basic_options();
        compare_parsed_result_to_expected_result(text_type, options);
    }

    #[test]
    fn test_wpa_cli_no_networks() {
        let text_type = NetworkTextType::WpaCliNoNetworks;
        let options = get_wpa_cli_basic_options();
        compare_parsed_result_to_expected_result(text_type, options);
    }

    #[test]
    fn test_wpa_cli_two_different_networks() {
        let text_type = NetworkTextType::WpaCliTwoDifferentNetworks;
        let options = get_wpa_cli_basic_options();
        compare_parsed_result_to_expected_result(text_type, options);
    }

    #[test]
    fn test_wpa_cli_seven_networks_two_duplicates_two_empty() {
        let text_type = NetworkTextType::WpaCliSevenNetworksTwoDuplicateTwoEmpty;
        let options = get_wpa_cli_basic_options();
        compare_parsed_result_to_expected_result(text_type, options);
    }

    #[test]
    fn test_wpa_cli_two_lines_missing_info() {
        let text_type = NetworkTextType::WpaCliTwoLinesMissingInfo;
        let options = get_wpa_cli_basic_options();
        compare_parsed_result_to_expected_result(text_type, options);
    }

    #[test]
    fn test_wpa_cli_two_networks_one_with_signal_level_parse_error() {
        let text_type = NetworkTextType::WpaCliTwoNetworksOneWithSignalLevelParseError;
        let options = get_wpa_cli_basic_options();
        compare_parsed_result_to_expected_result(text_type, options);
    }

    #[test]
    fn test_broken_input_two_words() {
        let text_type = NetworkTextType::BrokenInputTwoWords;
        let options = get_wpa_cli_basic_options();
        compare_parsed_result_to_expected_result(text_type, options);
    }

    // TODO: parsing failure
    // TODO: busy interface is retried
}
