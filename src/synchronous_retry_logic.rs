use crate::options::interfaces::*;
use crate::structs::*;

pub(crate) fn should_auto_retry_with_synchronous_scan<O>(
    options: &O,
    networks: &[AnnotatedWirelessNetwork],
) -> bool
where
    O: Global + AutoSelect,
{
    networks.is_empty()
        || match options.get_auto_mode() {
            AutoMode::KnownOrAsk | AutoMode::KnownOrFail => !networks.iter().any(|x| x.known),
            AutoMode::First | AutoMode::Ask => false,
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::wifi::connect::WifiConnectOptions;
    use crate::options::wifi::WifiOptions;

    fn get_options(auto_mode: &AutoMode) -> WifiConnectOptions {
        WifiConnectOptions::builder()
            .wifi(WifiOptions::default())
            .auto_mode(auto_mode.clone())
            .build()
    }

    fn get_three_unknown_networks() -> Vec<AnnotatedWirelessNetwork> {
        vec![
            AnnotatedWirelessNetwork {
                known: false,
                ..AnnotatedWirelessNetwork::default()
            },
            AnnotatedWirelessNetwork {
                known: false,
                ..AnnotatedWirelessNetwork::default()
            },
            AnnotatedWirelessNetwork {
                known: false,
                ..AnnotatedWirelessNetwork::default()
            },
        ]
    }

    fn get_three_known_networks() -> Vec<AnnotatedWirelessNetwork> {
        vec![
            AnnotatedWirelessNetwork {
                known: true,
                ..AnnotatedWirelessNetwork::default()
            },
            AnnotatedWirelessNetwork {
                known: true,
                ..AnnotatedWirelessNetwork::default()
            },
            AnnotatedWirelessNetwork {
                known: true,
                ..AnnotatedWirelessNetwork::default()
            },
        ]
    }

    fn get_empty_networks() -> Vec<AnnotatedWirelessNetwork> {
        vec![]
    }

    fn get_networks(network_list_type: &NetworkListType) -> Vec<AnnotatedWirelessNetwork> {
        match network_list_type {
            NetworkListType::ContainsKnown => get_three_known_networks(),
            NetworkListType::ContainsOnlyUnknown => get_three_unknown_networks(),
            NetworkListType::Empty => get_empty_networks(),
        }
    }

    enum NetworkListType {
        ContainsKnown,
        ContainsOnlyUnknown,
        Empty,
    }

    struct SyncTestDataProvider {
        network_list_type: NetworkListType,
        auto_mode: AutoMode,
        expected_should_retry: bool,
    }

    impl SyncTestDataProvider {
        fn new(
            network_list_type: NetworkListType,
            auto_mode: AutoMode,
            expected_should_retry: bool,
        ) -> Self {
            Self {
                network_list_type,
                auto_mode,
                expected_should_retry,
            }
        }
    }

    // If you aren't familiar with the data provider paradigm, it's simply a way to pass all
    // possible inputs and outputs for a function through unit tests, allowing us to easily
    // get 100% branch coverage for important logic, without having to write
    // `num_possibilities_for_input1 * num_possibilities_for_input2 * num_possible_outputs`
    // different tests. Instead, you just add conditions to the
    // dataprovider struct and matcher above, and update the expected logic here.
    fn get_data_providers() -> Vec<SyncTestDataProvider> {
        // To keep everything on one line:
        type STDP = SyncTestDataProvider;
        type NLT = NetworkListType;
        type AM = AutoMode;

        vec![
            STDP::new(NLT::ContainsKnown, AM::Ask, false),
            STDP::new(NLT::ContainsKnown, AM::First, false),
            STDP::new(NLT::ContainsKnown, AM::KnownOrAsk, false),
            STDP::new(NLT::ContainsKnown, AM::KnownOrFail, false),
            //
            STDP::new(NLT::ContainsOnlyUnknown, AM::Ask, false),
            STDP::new(NLT::ContainsOnlyUnknown, AM::First, false),
            STDP::new(NLT::ContainsOnlyUnknown, AM::KnownOrAsk, true),
            STDP::new(NLT::ContainsOnlyUnknown, AM::KnownOrFail, true),
            //
            STDP::new(NLT::Empty, AM::Ask, true),
            STDP::new(NLT::Empty, AM::First, true),
            STDP::new(NLT::Empty, AM::KnownOrAsk, true),
            STDP::new(NLT::Empty, AM::KnownOrFail, true),
        ]
    }

    #[test]
    fn test_should_synchronous_retry_all_unknown() {
        for SyncTestDataProvider {
            network_list_type,
            auto_mode,
            expected_should_retry,
        } in &get_data_providers()
        {
            let options = get_options(auto_mode);
            let networks = get_networks(network_list_type);
            let should_retry = should_auto_retry_with_synchronous_scan(&options, &networks);

            // Only bother trying to print if we know we're going to fail:
            if expected_should_retry != &should_retry {
                dbg!(&options, &networks, &should_retry);
            }

            assert_eq![expected_should_retry, &should_retry];
        }
    }
}
