use crate::structs::*;
use regex::Regex;

type FullParseResult = Result<ParseResult, ParseError>;

pub fn parse_result(_options: Options, scan_result: ScanResult) -> FullParseResult {
    // TODO: if scan type isn't specified, and parsing or scanning fails, try another scan type
    // TODO: implement
    match &scan_result.scan_type {
        ScanType::IW => Err(ParseError::NotImplemented),
        ScanType::IWList => Err(ParseError::NotImplemented),
        ScanType::WpaCli => parse_wpa_cli_scan(scan_result.output),
    }
}

pub fn split_iw_output_into_chunks(output: String) -> Vec<Vec<String>> {
    let lines = output
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let bss = Regex::new("^BSS ((\\w\\w:){5}\\w\\w)").unwrap();

    let mut iw_network_line_groups = vec![];

    let mut linenum = 0;

    while linenum < lines.len() {
        let mut line_group_lines = vec![];

        line_group_lines.push(lines[linenum].clone());
        //linenum += 1;

        while linenum < lines.len() {
            let line = lines[linenum].trim();
            let lawl = bss.is_match(line);

            println!("{:?} {}", lawl, line);
            linenum += 1;
        }

        iw_network_line_groups.push(line_group_lines);
    }

    vec![]
    //    lines = iw_scan_output.split("\n")
    //
    //    iw_network_line_groups = []
    //    linenum = 0
    //    while linenum < len(lines):
    //        iw_network_line_groups.append([])
    //        network = iw_network_line_groups[-1]
    //
    //        network.append(lines[linenum])
    //        linenum += 1
    //
    //        while linenum < len(lines):
    //            line = lines[linenum]
    //            if re.match(IW_NETWORK_START_REGEX, line):
    //                break
    //            else:
    //                network.append(line)
    //                linenum += 1
    //
    //    return iw_network_line_groups
}

pub fn parse_wpa_cli_scan(output: String) -> FullParseResult {
    let mut lines = output.lines().map(|x| x.to_string());
    let mut networks = vec![];
    let mut line_parse_errors = vec![];
    let _header1 = lines.next().ok_or(ParseError::MissingWpaCliHeader)?;
    let _header2 = lines.next().ok_or(ParseError::MissingWpaCliHeader)?;
    for line in lines {
        let res = parse_wpa_line_into_network(line.to_string());
        match res {
            Ok(nw) => networks.push(nw),
            Err(err) => line_parse_errors.push((line, err)),
        };
    }

    Ok(ParseResult {
        scan_type: ScanType::WpaCli,
        seen_networks: networks,
        line_parse_errors: line_parse_errors,
    })
}

pub fn parse_wpa_line_into_network(line: String) -> Result<WirelessNetwork, IndividualParseError> {
    let mut fields = line.split_ascii_whitespace();

    let fieldmiss = IndividualParseError::MissingWpaCliResultField;
    let bssid = fields.next().ok_or(fieldmiss)?;
    let _freq = fields.next().ok_or(fieldmiss)?;
    let signal_level = fields.next().ok_or(fieldmiss)?;
    let flags = fields.next().ok_or(fieldmiss)?;

    // NOTE: Broken for SSIDs with multiple sequential spaces!
    let essid = fields
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    let wpa = flags.contains("WPA");
    let signal_strength = signal_level
        .parse::<i32>()
        .or(Err(IndividualParseError::FailedToParseSignalLevel))?;

    Ok(WirelessNetwork {
        essid: essid.to_string(),
        wpa: wpa,
        bssid: Some(bssid.to_string()),
        signal_strength: Some(signal_strength),
        channel_utilisation: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // pub static IW_ONE_NETWORK: &'static str = include_str!("samples/iw_one_network.txt");
    // pub static IW_TWO_DIFFERENT_NETWORKS: &'static str =
    //  include_str!("samples/iw_two_different_networks.txt");
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

    fn get_wpa_cli_basic_options() -> Options {
        Options {
            scan_type: ScanType::WpaCli,
            interface: "some_fake_name".to_string(),
            selection_method: SelectionMethod::Dmenu,
            output_types: vec![],
            connect_via: None,
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    enum NetworkTextType {
        // IWOneNetwork,
        // IWTwoDifferentNetworks,
        WpaCliTwoDifferentNetworks,
        WpaCliSevenNetworksTwoDuplicateTwoEmpty,
        WpaCliTwoLinesMissingInfo,
        WpaCliTwoNetworksOneWithSignalLevelParseError,
        BrokenInputTwoWords,
    }

    fn get_text(text_type: NetworkTextType) -> String {
        match text_type {
            // NetworkTextType::IWOneNetwork => self::IW_ONE_NETWORK.to_string(),
            // NetworkTextType::IWTwoDifferentNetworks => self::IW_TWO_DIFFERENT_NETWORKS.to_string(),
            NetworkTextType::WpaCliTwoDifferentNetworks => {
                self::WPA_CLI_TWO_DIFFERENT_NETWORKS.to_string()
            }
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

    fn get_expected_result(text_type: NetworkTextType) -> FullParseResult {
        match text_type {
            // NetworkTextType::IWOneNetwork => Ok(ParseResult {
            //     scan_type: ScanType::IW,
            //     seen_networks: vec![WirelessNetwork {
            //         essid: "Pee Pee Poo Poo Man".to_string(),
            //         wpa: true,
            //         bssid: Some("32:ac:a3:7b:ab:0b".to_string()),
            //         signal_strength: Some(-38),
            //         channel_utilisation: None,
            //     }],
            // }),
            // NetworkTextType::IWTwoDifferentNetworks => Ok(ParseResult {
            //     scan_type: ScanType::IW,
            //     seen_networks: vec![
            //         WirelessNetwork {
            //             essid: "Valparaiso_Guest_House 1".to_string(),
            //             wpa: true,
            //             bssid: Some("f4:28:53:fe:a5:d0".to_string()),
            //             signal_strength: Some(-65),
            //             channel_utilisation: None,
            //         },
            //         WirelessNetwork {
            //             essid: "Valparaiso_Guest_House 2".to_string(),
            //             wpa: true,
            //             bssid: Some("68:72:51:68:73:da".to_string()),
            //             signal_strength: Some(-46),
            //             channel_utilisation: None,
            //         },
            //     ],
            // }),
            NetworkTextType::WpaCliTwoDifferentNetworks => Ok(ParseResult {
                scan_type: ScanType::WpaCli,
                seen_networks: vec![
                    WirelessNetwork {
                        essid: "Valparaiso_Guest_House 1".to_string(),
                        wpa: true,
                        bssid: Some("f4:28:53:fe:a5:d0".to_string()),
                        signal_strength: Some(-66),
                        channel_utilisation: None,
                    },
                    WirelessNetwork {
                        essid: "Valparaiso_Guest_House 2".to_string(),
                        wpa: true,
                        bssid: Some("68:72:51:68:73:da".to_string()),
                        signal_strength: Some(-47),
                        channel_utilisation: None,
                    },
                ],
                line_parse_errors: vec![],
            }),
            NetworkTextType::WpaCliSevenNetworksTwoDuplicateTwoEmpty => Ok(ParseResult {
                scan_type: ScanType::WpaCli,
                seen_networks: vec![
                    WirelessNetwork {
                        essid: "Nima Lodge".to_string(),
                        wpa: true,
                        bssid: Some("78:8a:20:e3:9d:62".to_string()),
                        signal_strength: Some(-41),
                        channel_utilisation: None,
                    },
                    WirelessNetwork {
                        essid: "DIRECT-AF-HP DeskJet 3830 series".to_string(),
                        wpa: true,
                        bssid: Some("fc:3f:db:a1:5e:b0".to_string()),
                        signal_strength: Some(-68),
                        channel_utilisation: None,
                    },
                    WirelessNetwork {
                        essid: "Nima Lodge".to_string(),
                        wpa: true,
                        bssid: Some("fc:ec:da:69:e0:3e".to_string()),
                        signal_strength: Some(-85),
                        channel_utilisation: None,
                    },
                    WirelessNetwork {
                        essid: "".to_string(),
                        wpa: true,
                        bssid: Some("fe:ec:da:69:e0:3e".to_string()),
                        signal_strength: Some(-85),
                        channel_utilisation: None,
                    },
                    WirelessNetwork {
                        essid: "WISPerNET-George-Sentraal1".to_string(),
                        wpa: false,
                        bssid: Some("ba:69:f4:1f:2d:15".to_string()),
                        signal_strength: Some(-89),
                        channel_utilisation: None,
                    },
                    WirelessNetwork {
                        essid: "WISPerNET-Bosplaas-SW-802.11".to_string(),
                        wpa: false,
                        bssid: Some("b8:69:f4:1f:2d:15".to_string()),
                        signal_strength: Some(-89),
                        channel_utilisation: None,
                    },
                    WirelessNetwork {
                        essid: "".to_string(),
                        wpa: true,
                        bssid: Some("7a:8a:20:e3:9d:62".to_string()),
                        signal_strength: Some(-39),
                        channel_utilisation: None,
                    },
                ],
                line_parse_errors: vec![],
            }),
            NetworkTextType::WpaCliTwoLinesMissingInfo => Ok(ParseResult {
                scan_type: ScanType::WpaCli,
                seen_networks: vec![],
                line_parse_errors: vec![
                    (
                        "f4:28:53:fe:a5:d0\t2437\t-66\t".to_string(),
                        IndividualParseError::MissingWpaCliResultField,
                    ),
                    (
                        "68:72:51:68:73:da\t2457\t".to_string(),
                        IndividualParseError::MissingWpaCliResultField,
                    ),
                ],
            }),
            NetworkTextType::WpaCliTwoNetworksOneWithSignalLevelParseError => Ok(ParseResult {
                scan_type: ScanType::WpaCli,
                seen_networks: vec![WirelessNetwork {
                    essid: "Valparaiso_Guest_House 1".to_string(),
                    wpa: true,
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
            NetworkTextType::BrokenInputTwoWords => Err(ParseError::MissingWpaCliHeader),
        }
    }

    fn compare_parsed_result_to_expected_result(text_type: NetworkTextType, options: Options) {
        let contents = get_text(text_type);
        let full_expected_result = get_expected_result(text_type);

        let scan_result = ScanResult {
            scan_type: options.scan_type.clone(),
            output: contents,
        };

        let full_parse_result = parse_result(options, scan_result);

        // TODO: match here - if both aren't errors, unwrap and compare
        //                    if both are errors, make sure they're the same
        match (&full_parse_result, &full_expected_result) {
            (Ok(parse_result), Ok(expected_result)) => {
                assert_eq![parse_result, expected_result];
            }
            (Err(parse_error), Err(expected_error)) => {
                assert_eq![parse_error, expected_error];
            }
            (_, _) => {
                println!("Full parse result: {:?}", full_parse_result);
                println!("Expt parse result: {:?}", full_expected_result);
                assert![false];
            }
        }
    }

    // Broken
    // #[test]
    // fn test_single_iw_result() {
    //     let text_type = NetworkTextType::IWOneNetwork;
    //     let options = Options {
    //         scan_type: ScanType::IW,
    //         interface: "some_fake_name".to_string(),
    //         output_types: vec![],
    //         connect_via: None,
    //     };
    //
    //     compare_parsed_result_to_expected_result(text_type, options);
    // }

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
