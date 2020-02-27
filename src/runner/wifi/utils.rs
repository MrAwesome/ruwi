use std::thread;

use crate::annotate_networks::annotate_networks;
use crate::check_known_identifiers::KnownIdentifiers;
use crate::enums::*;
use crate::errors::*;
use crate::find_known_network_names::find_known_network_names;
use crate::options::interfaces::*;
use crate::parse::parse_result;
use crate::utils::*;
use crate::sort_networks::SortedFilteredNetworks;
use crate::structs::*;
use crate::synchronous_retry_logic::should_auto_retry_with_synchronous_scan;
use crate::wifi_scan::wifi_scan;

const LOOP_MAX: u16 = 1000;

pub(crate) fn get_selected_network<O>(options: &O) -> Result<AnnotatedWirelessNetwork, RuwiError>
where
    O: Send + Sync + Global + LinuxNetworkingInterface + AutoSelect + WifiDataGatherer,
{
    let mut synchronous_retry = None;
    let mut loop_protection = 0;
    loop {
        loop_check(&mut loop_protection, LOOP_MAX)?;
        let (known_network_names, scan_result) = options.get_wifi_data(&synchronous_retry)?;
        let parse_results = parse_result(options, &scan_result)?;

        let annotated_networks =
            annotate_networks(options, &parse_results.seen_networks, &known_network_names);
        if should_auto_retry_with_synchronous_scan(options, &annotated_networks, &synchronous_retry)
        {
            synchronous_retry = Some(SynchronousRescanType::Automatic);
            continue;
        }

        let sorted_networks = SortedFilteredNetworks::new(annotated_networks);

        let selected_network_result = sorted_networks.select_network(options);
        if manual_refresh_requested(&selected_network_result) {
            synchronous_retry = Some(SynchronousRescanType::ManuallyRequested);
            continue;
        }
        return selected_network_result;
    }
}

pub(super) fn gather_wifi_network_data<O>(
    options: &O,
    synchronous_rescan: &Option<SynchronousRescanType>,
) -> Result<(KnownIdentifiers, ScanResult), RuwiError>
where
    O: 'static + Global + Wifi + WifiConnect + LinuxNetworkingInterface + Send + Sync + Clone,
{
    let options: &'static O = Box::leak(Box::new(options.clone()));
    let synchronous_rescan = synchronous_rescan.clone();

    let get_nw_names = thread::spawn(move || find_known_network_names(options));
    let get_scan_results = thread::spawn(move || wifi_scan(options, &synchronous_rescan));

    let known_network_names = await_thread(get_nw_names)??;
    let scan_result = await_thread(get_scan_results)??;

    Ok((known_network_names, scan_result))
}

pub(super) fn manual_refresh_requested<T>(res: &Result<T, RuwiError>) -> bool {
    if let Err(err) = res {
        if err.kind == RuwiErrorKind::RefreshRequested {
            return true;
        }
    }
    false
}

pub(super) fn get_network_from_given_essid<O>(
    options: &O,
    essid: &str,
) -> Result<AnnotatedWirelessNetwork, RuwiError>
where
    O: Global + Wifi + WifiConnect + LinuxNetworkingInterface,
{
    let is_known = find_known_network_names(options)?.check_for(essid);
    let is_encrypted = options.get_given_encryption_key().is_some();
    Ok(AnnotatedWirelessNetwork::from_essid(
        essid.into(),
        is_known,
        is_encrypted,
    ))
}

