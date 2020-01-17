mod nmcli;
use nmcli::parse_nmcli_scan;

use crate::rerr;
use crate::structs::*;

use regex::Regex;
use unescape::unescape;

pub(crate) fn parse_result(
    options: &Options,
    scan_result: &ScanResult,
) -> Result<ParseResult, RuwiError> {
    let st = scan_result.scan_type.clone();
    let res = match &st {
        WifiScanType::Nmcli => parse_nmcli_scan(options, &scan_result.scan_output, st),
        WifiScanType::WpaCli => parse_wpa_cli_scan(options, &scan_result.scan_output, st),
        WifiScanType::IW => parse_iw_scan(options, &scan_result.scan_output, st),
        WifiScanType::RuwiJSON => Err(nie("JSON support is coming soon!")),
    };

    if options.debug {
        dbg![&res];
    }
    res
}

fn parse_iw_scan(
    options: &Options,
    output: &str,
    scan_type: WifiScanType,
) -> Result<ParseResult, RuwiError> {
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
        scan_type,
        seen_networks,
        line_parse_errors,
    })
}

fn break_iw_output_into_chunks_per_network<'a>(
    options: &Options,
    output: &'a str,
) -> Result<Vec<Vec<&'a str>>, RuwiError> {
    let mut lines = output.trim().lines().map(str::trim);
    let mut iw_network_line_groups = vec![];

    let bss_re = get_iw_bss_regex();

    let mut network = vec![];

    if let Some(line) = lines.next() {
        if bss_re.is_match(&line) {
            network.push(line);
        } else {
            return Err(err_iw_malformed_output(options));
        }
    } else {
        return Err(err_iw_no_networks_seen(options));
    }

    loop {
        if let Some(line) = lines.next() {
            if bss_re.is_match(&line) {
                iw_network_line_groups.push(network);
                network = vec![];
            }

            network.push(line);
        } else {
            iw_network_line_groups.push(network);
            break;
        }
    }
    Ok(iw_network_line_groups)
}

fn parse_iw_chunk_into_network(chunk: &[&str]) -> Result<WirelessNetwork, IndividualParseError> {
    let essid = unescape(
        chunk
            .iter()
            .find_map(|line| if line.starts_with("SSID: ") { Some(line.trim_start_matches("SSID: ")) } else { None })
            .ok_or(IndividualParseError::MissingIWSSIDField)?,
    )
    .ok_or(IndividualParseError::FailedToUnescapeSSIDField)?;

    let is_encrypted = chunk
        .iter()
        .find(|line| line.starts_with("capability:"))
        .ok_or(IndividualParseError::MissingIWCapabilityField)?
        .split_ascii_whitespace()
        .any(|x| x == "Privacy");

    let bssid = chunk
        .first()
        .ok_or(IndividualParseError::ZeroLengthIWChunk)?
        .trim_start_matches("BSS ")
        .split('(')
        .map(|x| x.into())
        .next();

    let signal_strength = chunk
        .iter()
        .filter(|line| line.starts_with("signal: "))
        .find_map(|line| {
            line.trim_start_matches("signal: ")
                .trim_end_matches(" dBm")
                .split('.')
                .next()?
                .parse::<i32>()
                .ok()
        })
        .map(|x| x + 90);

    Ok(WirelessNetwork {
        essid,
        is_encrypted,
        bssid,
        signal_strength,
        ..WirelessNetwork::default()
    })
}

fn get_iw_bss_regex() -> Regex {
    Regex::new(r"^BSS ((\w\w:){5}\w\w)").expect("Failure creating regex for iw parsing...")
}

fn err_iw_malformed_output(options: &Options) -> RuwiError {
    rerr!(
        RuwiErrorKind::MalformedIWOutput,
        format!(
            "Malformed output returned by `sudo iw {} scan dump`. Try running it manually.",
            options.interface
        )
    )
}

fn err_iw_no_networks_seen(options: &Options) -> RuwiError {
    rerr!(
        RuwiErrorKind::NoNetworksSeenWithIWScanDump,
        format!("No networks seen by `sudo iw {} scan dump`. Are you near wireless networks? Try running `sudo iw {} scan`.", 
            options.interface, 
            options.interface))
}

fn parse_wpa_cli_scan(
    _options: &Options,
    output: &str,
    scan_type: WifiScanType,
) -> Result<ParseResult, RuwiError> {
    let mut lines = output.trim().lines().map(ToString::to_string);
    let mut networks = vec![];
    let mut line_parse_errors = vec![];

    let _header1 = lines.next().ok_or_else(missing_wpa_cli_header)?;
    let _header2 = lines.next().ok_or_else(missing_wpa_cli_header)?;

    for line in lines {
        let res = parse_wpa_line_into_network(&line);
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
            scan_type,
            seen_networks: networks,
            line_parse_errors,
        })
    }
}

fn err_wpa_cli_no_networks_seen() -> RuwiError {
    rerr!(
        RuwiErrorKind::NoNetworksSeenWithWPACliScanResults,
        "No networks seen by `sudo wpa_cli scan_results`. Are you near wireless networks? Try running `sudo wpa_cli scan`.")
}

fn missing_wpa_cli_header() -> RuwiError {
    rerr!(
        RuwiErrorKind::WPACliHeaderMalformedOrMissing,
        "`wpa_cli scan_results` header malformed or missing"
    )
}

fn parse_wpa_line_into_network(line: &str) -> Result<WirelessNetwork, IndividualParseError> {
    let mut fields = line.split_ascii_whitespace();

    let fieldmiss = IndividualParseError::MissingWpaCliResultField;
    let bssid = fields.next().ok_or(fieldmiss)?;
    let _freq = fields.next().ok_or(fieldmiss)?;
    let signal_level = fields.next().ok_or(fieldmiss)?;
    let flags = fields.next().ok_or(fieldmiss)?;

    // NOTE: Broken for SSIDs with multiple sequential spaces, or trailing whitespace
    let essid = fields
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(" ");

    let is_encrypted = flags.contains("WPA");
    let signal_strength = signal_level
        .parse::<i32>()
        .map(|x| x + 90)
        .or(Err(IndividualParseError::FailedToParseSignalLevel))?;

    Ok(WirelessNetwork {
        essid,
        is_encrypted,
        bssid: Some(bssid.to_string()),
        signal_strength: Some(signal_strength),
        ..WirelessNetwork::default()
    })
}

// TODO: for networkmanager: shorten to nm in options
// TODO: check behavior of SSIDs with colons in them for scan/parse
// TODO: check behavior on nothing returned from scan
#[cfg(test)]
mod tests {
    use super::*;

    fn compare_parsed_result_to_expected_result(options: &Options, 
        scan_result: &ScanResult,
        expected_parse_result: &Result<ParseResult, RuwiError>,
    ) {

        let actual_parse_result = parse_result(&options, &scan_result);

        dbg!(&expected_parse_result);
        dbg!(&actual_parse_result);

        match (&expected_parse_result, &actual_parse_result) {
            (Ok(expected_result), Ok(parse_result)) => {
                assert_eq![expected_result, parse_result];
            }
            (Err(expected_error), Err(parse_error)) => {
                let expct_desc = expected_error.to_string();
                let parse_desc = parse_error.to_string();
                assert_eq![expct_desc, parse_desc];
            }
            (_, _) => {
                panic!("Actual and expected parse results differed!");
            }
        }
    }

    #[test]
    fn test_iw_one_network() {
        let st = WifiScanType::IW;
        let options = Options::from_scan_type(st.clone());
        let scan_result = ScanResult {
            scan_type: st.clone(),
            scan_output: include_str!("samples/iw_one_network.txt").to_string(),
        };
        let expected_parse_result = Ok(ParseResult {
            scan_type: st,
            seen_networks: vec![WirelessNetwork {
                essid: "Pee Pee Poo Poo Man".to_string(),
                is_encrypted: true,
                bssid: Some("32:ac:a3:7b:ab:0b".to_string()),
                signal_strength: Some(52),
                ..WirelessNetwork::default()
            }],
            line_parse_errors: vec![],
        });
        compare_parsed_result_to_expected_result(&options, &scan_result, &expected_parse_result);
    }

    #[test]
    fn test_iw_two_different_networks() {
        let st = WifiScanType::IW;
        let options = Options::from_scan_type(st.clone());
        let scan_result = ScanResult {
            scan_type: st.clone(),
            scan_output: include_str!("samples/iw_two_different_networks.txt").to_string(),
        };
        let expected_parse_result = Ok(ParseResult {
            scan_type: st,
            seen_networks: vec![
                WirelessNetwork {
                    essid: "Valparaiso_Guest_House 1".to_string(),
                    is_encrypted: true,
                    bssid: Some("f4:28:53:fe:a5:d0".to_string()),
                    signal_strength: Some(25),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "Valparaiso_Guest_House 2".to_string(),
                    is_encrypted: true,
                    bssid: Some("68:72:51:68:73:da".to_string()),
                    signal_strength: Some(44),
                    ..WirelessNetwork::default()
                },
            ],
            line_parse_errors: vec![],
        });
        compare_parsed_result_to_expected_result(&options, &scan_result, &expected_parse_result);
    }

    #[test]
    fn test_wpa_cli_no_networks() {
        let st = WifiScanType::WpaCli;
        let options = Options::from_scan_type(st.clone());
        let scan_result = ScanResult {
            scan_type: st,
            scan_output: include_str!("samples/wpa_cli_no_networks.txt").to_string(),
        };
        let expected_parse_result = Err(err_wpa_cli_no_networks_seen());
        compare_parsed_result_to_expected_result(&options, &scan_result, &expected_parse_result);
    }

    #[test]
    fn test_wpa_cli_two_different_networks() {
        let st = WifiScanType::WpaCli;
        let options = Options::from_scan_type(st.clone());
        let scan_result = ScanResult {
            scan_type: st.clone(),
            scan_output: include_str!("samples/wpa_cli_two_different_networks.txt").to_string(),
        };
        let expected_parse_result = Ok(ParseResult {
            scan_type: st,
            seen_networks: vec![
                WirelessNetwork {
                    essid: "Valparaiso_Guest_House 1".to_string(),
                    is_encrypted: true,
                    bssid: Some("f4:28:53:fe:a5:d0".to_string()),
                    signal_strength: Some(24),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "Valparaiso_Guest_House 2".to_string(),
                    is_encrypted: true,
                    bssid: Some("68:72:51:68:73:da".to_string()),
                    signal_strength: Some(43),
                    ..WirelessNetwork::default()
                },
            ],
            line_parse_errors: vec![],
        });
        compare_parsed_result_to_expected_result(&options, &scan_result, &expected_parse_result);
    }

    #[test]
    fn test_wpa_cli_seven_networks_two_duplicates_two_empty() {
        let st = WifiScanType::WpaCli;
        let options = Options::from_scan_type(st.clone());
        let scan_result = ScanResult {
            scan_type: st.clone(),
            scan_output: include_str!("samples/wpa_cli_seven_networks_two_duplicates_two_empty.txt").to_string(),
        };
        let expected_parse_result = Ok(ParseResult {
            scan_type: st,
            seen_networks: vec![
                WirelessNetwork {
                    essid: "Nima Lodge".to_string(),
                    is_encrypted: true,
                    bssid: Some("78:8a:20:e3:9d:62".to_string()),
                    signal_strength: Some(49),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "DIRECT-AF-HP DeskJet 3830 series".to_string(),
                    is_encrypted: true,
                    bssid: Some("fc:3f:db:a1:5e:b0".to_string()),
                    signal_strength: Some(22),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "Nima Lodge".to_string(),
                    is_encrypted: true,
                    bssid: Some("fc:ec:da:69:e0:3e".to_string()),
                    signal_strength: Some(5),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "".to_string(),
                    is_encrypted: true,
                    bssid: Some("fe:ec:da:69:e0:3e".to_string()),
                    signal_strength: Some(5),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "WISPerNET-George-Sentraal1".to_string(),
                    is_encrypted: false,
                    bssid: Some("ba:69:f4:1f:2d:15".to_string()),
                    signal_strength: Some(1),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "WISPerNET-Bosplaas-SW-802.11".to_string(),
                    is_encrypted: false,
                    bssid: Some("b8:69:f4:1f:2d:15".to_string()),
                    signal_strength: Some(1),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "".to_string(),
                    is_encrypted: true,
                    bssid: Some("7a:8a:20:e3:9d:62".to_string()),
                    signal_strength: Some(51),
                    ..WirelessNetwork::default()
                },
            ],
            line_parse_errors: vec![],
        });
        compare_parsed_result_to_expected_result(&options, &scan_result, &expected_parse_result);
    }

    #[test]
    fn test_wpa_cli_two_lines_missing_info() {
        let st = WifiScanType::WpaCli;
        let options = Options::from_scan_type(st.clone());
        let scan_result = ScanResult {
            scan_type: st.clone(),
            scan_output: include_str!("samples/wpa_cli_two_lines_missing_info.txt").to_string(),
        };
        let expected_parse_result = Ok(ParseResult {
            scan_type: st,
            seen_networks: vec![WirelessNetwork {
                essid: "Valparaiso_Guest_House 1".to_string(),
                is_encrypted: true,
                bssid: Some("f4:28:53:fe:a5:d0".to_string()),
                signal_strength: Some(24),
                ..WirelessNetwork::default()
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
        });
        compare_parsed_result_to_expected_result(&options, &scan_result, &expected_parse_result);
    }

    #[test]
    fn test_wpa_cli_two_networks_one_with_signal_level_parse_error() {
        let st = WifiScanType::WpaCli;
        let options = Options::from_scan_type(st.clone());
        let scan_result = ScanResult {
            scan_type: st.clone(),
            scan_output: include_str!("samples/wpa_cli_two_networks_one_with_signal_level_parse_error.txt").to_string(),
        };
        let expected_parse_result = Ok(ParseResult {
            scan_type: st,
            seen_networks: vec![WirelessNetwork {
                essid: "Valparaiso_Guest_House 1".to_string(),
                is_encrypted: true,
                bssid: Some("f4:28:53:fe:a5:d0".to_string()),
                signal_strength: Some(24),
                ..WirelessNetwork::default()
            }],
            line_parse_errors: vec![(
                "68:72:51:68:73:da\t2457\t-xx\t[WPA2-PSK-CCMP][ESS]\tValparaiso_Guest_House 2"
                    .to_string(),
                IndividualParseError::FailedToParseSignalLevel,
            )],
        });
        compare_parsed_result_to_expected_result(&options, &scan_result, &expected_parse_result);
    }

    #[test]
    fn test_wpa_cli_broken_input_two_words() {
        let st = WifiScanType::WpaCli;
        let options = Options::from_scan_type(st.clone());
        let scan_result = ScanResult {
            scan_type: st,
            scan_output: include_str!("samples/broken_input_two_words.txt").to_string(),
        };
        let expected_parse_result = Err(missing_wpa_cli_header());
        compare_parsed_result_to_expected_result(&options, &scan_result, &expected_parse_result);
    }

    #[test]
    fn test_nmcli_many_networks() {
        let st = WifiScanType::Nmcli;
        let options = Options::default();
        let scan_result = ScanResult {
            scan_type: st.clone(),
            scan_output: include_str!("samples/nmcli_many_networks.txt").to_string(),
        };
        let expected_parse_result = Ok(ParseResult {
            scan_type: st,
            seen_networks: vec![
                WirelessNetwork {
                    essid: "alltheinternets".to_string(),
                    is_encrypted: true,
                    signal_strength: Some(95),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "Patrician Pad".to_string(),
                    is_encrypted: true,
                    signal_strength: Some(95),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "casa".to_string(),
                    is_encrypted: true,
                    signal_strength: Some(94),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "Patrician Pad".to_string(),
                    is_encrypted: true,
                    signal_strength: Some(94),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "xfinitywifi".to_string(),
                    is_encrypted: false,
                    signal_strength: Some(90),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "MeshResearch".to_string(),
                    is_encrypted: true,
                    signal_strength: Some(52),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "".to_string(),
                    is_encrypted: true,
                    signal_strength: Some(35),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "Lots:of:colons:lol:".to_string(),
                    is_encrypted: false,
                    signal_strength: Some(34),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "".to_string(),
                    is_encrypted: true,
                    signal_strength: Some(32),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "xfinitywifi".to_string(),
                    is_encrypted: false,
                    signal_strength: Some(32),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "Okonomiyaki".to_string(),
                    is_encrypted: true,
                    signal_strength: Some(30),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "XFINITY".to_string(),
                    is_encrypted: true,
                    signal_strength: Some(17),
                    ..WirelessNetwork::default()
                },
                WirelessNetwork {
                    essid: "".to_string(),
                    is_encrypted: true,
                    signal_strength: Some(15),
                    ..WirelessNetwork::default()
                },
            ],
            line_parse_errors: vec![],
        });
        compare_parsed_result_to_expected_result(&options, &scan_result, &expected_parse_result);
    }
}
