use crate::common::*;

pub(crate) fn should_auto_retry_with_synchronous_scan<O>(
    options: &O,
    networks: &[AnnotatedWirelessNetwork],
    synchronous_retry: &Option<SynchronousRescanType>,
) -> bool
where
    O: Global + AutoSelect,
{
    synchronous_retry.is_none()
        && (networks.is_empty()
            || match options.get_auto_mode() {
                AutoMode::KnownOrAsk | AutoMode::KnownOrFail => !networks.iter().any(|x| x.is_known()),
                AutoMode::First | AutoMode::Ask => false,
            })
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
            AnnotatedWirelessNetwork::builder()
                .essid("first_nw")
                .service_identifier(None)
            .build(),
            AnnotatedWirelessNetwork::builder()
                .essid("second_nw")
                .service_identifier(None)
            .build(),
            AnnotatedWirelessNetwork::builder()
                .essid("third_nw")
                .service_identifier(None)
            .build(),
        ]
    }

    fn get_three_known_networks() -> Vec<AnnotatedWirelessNetwork> {
        vec![
            AnnotatedWirelessNetwork::builder()
                .essid("first_nw")
                .service_identifier(NetworkServiceIdentifier::netctl_nw("first_service_id"))
            .build(),
            AnnotatedWirelessNetwork::builder()
                .essid("second_nw")
                .service_identifier(NetworkServiceIdentifier::netctl_nw("second_service_id"))
            .build(),
            AnnotatedWirelessNetwork::builder()
                .essid("third_nw")
                .service_identifier(NetworkServiceIdentifier::netctl_nw("third_service_id"))
            .build(),
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
        synchronous_rescan: Option<SynchronousRescanType>,
        network_list_type: NetworkListType,
        auto_mode: AutoMode,
        expected_should_retry: bool,
    }

    impl SyncTestDataProvider {
        fn new(
            synchronous_rescan: Option<SynchronousRescanType>,
            network_list_type: NetworkListType,
            auto_mode: AutoMode,
            expected_should_retry: bool,
        ) -> Self {
            Self {
                synchronous_rescan,
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
            STDP::new(None, NLT::ContainsKnown, AM::Ask, false),
            STDP::new(None, NLT::ContainsKnown, AM::First, false),
            STDP::new(None, NLT::ContainsKnown, AM::KnownOrAsk, false),
            STDP::new(None, NLT::ContainsKnown, AM::KnownOrFail, false),
            //
            STDP::new(None, NLT::ContainsOnlyUnknown, AM::Ask, false),
            STDP::new(None, NLT::ContainsOnlyUnknown, AM::First, false),
            STDP::new(None, NLT::ContainsOnlyUnknown, AM::KnownOrAsk, true),
            STDP::new(None, NLT::ContainsOnlyUnknown, AM::KnownOrFail, true),
            //
            STDP::new(None, NLT::Empty, AM::Ask, true),
            STDP::new(None, NLT::Empty, AM::First, true),
            STDP::new(None, NLT::Empty, AM::KnownOrAsk, true),
            STDP::new(None, NLT::Empty, AM::KnownOrFail, true),
            //
            STDP::new(Some(SynchronousRescanType::Automatic), NLT::ContainsKnown, AM::Ask, false),
            STDP::new(Some(SynchronousRescanType::Automatic), NLT::ContainsKnown, AM::First, false),
            STDP::new(Some(SynchronousRescanType::Automatic), NLT::ContainsKnown, AM::KnownOrAsk, false),
            STDP::new(Some(SynchronousRescanType::Automatic), NLT::ContainsKnown, AM::KnownOrFail, false),
            //
            STDP::new(Some(SynchronousRescanType::Automatic), NLT::ContainsOnlyUnknown, AM::Ask, false),
            STDP::new(Some(SynchronousRescanType::Automatic), NLT::ContainsOnlyUnknown, AM::First, false),
            STDP::new(Some(SynchronousRescanType::Automatic), NLT::ContainsOnlyUnknown, AM::KnownOrAsk, false),
            STDP::new(Some(SynchronousRescanType::Automatic), NLT::ContainsOnlyUnknown, AM::KnownOrFail, false),
            //
            STDP::new(Some(SynchronousRescanType::Automatic), NLT::Empty, AM::Ask, false),
            STDP::new(Some(SynchronousRescanType::Automatic), NLT::Empty, AM::First, false),
            STDP::new(Some(SynchronousRescanType::Automatic), NLT::Empty, AM::KnownOrAsk, false),
            STDP::new(Some(SynchronousRescanType::Automatic), NLT::Empty, AM::KnownOrFail, false),
            //
            STDP::new(Some(SynchronousRescanType::ManuallyRequested), NLT::ContainsKnown, AM::Ask, false),
            STDP::new(Some(SynchronousRescanType::ManuallyRequested), NLT::ContainsKnown, AM::First, false),
            STDP::new(Some(SynchronousRescanType::ManuallyRequested), NLT::ContainsKnown, AM::KnownOrAsk, false),
            STDP::new(Some(SynchronousRescanType::ManuallyRequested), NLT::ContainsKnown, AM::KnownOrFail, false),
            //
            STDP::new(Some(SynchronousRescanType::ManuallyRequested), NLT::ContainsOnlyUnknown, AM::Ask, false),
            STDP::new(Some(SynchronousRescanType::ManuallyRequested), NLT::ContainsOnlyUnknown, AM::First, false),
            STDP::new(Some(SynchronousRescanType::ManuallyRequested), NLT::ContainsOnlyUnknown, AM::KnownOrAsk, false),
            STDP::new(Some(SynchronousRescanType::ManuallyRequested), NLT::ContainsOnlyUnknown, AM::KnownOrFail, false),
            //
            STDP::new(Some(SynchronousRescanType::ManuallyRequested), NLT::Empty, AM::Ask, false),
            STDP::new(Some(SynchronousRescanType::ManuallyRequested), NLT::Empty, AM::First, false),
            STDP::new(Some(SynchronousRescanType::ManuallyRequested), NLT::Empty, AM::KnownOrAsk, false),
            STDP::new(Some(SynchronousRescanType::ManuallyRequested), NLT::Empty, AM::KnownOrFail, false),
        ]
    }

    #[test]
    fn test_should_synchronous_retry_all_unknown() {
        for SyncTestDataProvider {
            synchronous_rescan,
            network_list_type,
            auto_mode,
            expected_should_retry,
        } in &get_data_providers()
        {
            let options = get_options(auto_mode);
            let networks = get_networks(network_list_type);
            let should_retry = should_auto_retry_with_synchronous_scan(&options, &networks, synchronous_rescan);

            // Only bother trying to print if we know we're going to fail:
            if expected_should_retry != &should_retry {
                dbg!(&options, &networks, &should_retry);
            }

            assert_eq![expected_should_retry, &should_retry];
        }
    }
}
