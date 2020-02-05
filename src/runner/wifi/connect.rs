use std::thread;

use crate::annotate_networks::annotate_networks;
use crate::configure_network::possibly_configure_network;
use crate::connect::connect_to_network;
use crate::encryption_key::possibly_get_encryption_key;
use crate::errors::*;
use crate::find_known_network_names::find_known_network_names;
use crate::options::interfaces::*;
use crate::parse::parse_result;
use crate::rerr;
use crate::select_network::select_network;
use crate::sort_networks::sort_and_filter_networks;
use crate::structs::*;
use crate::synchronous_retry_logic::should_retry_with_synchronous_scan;
use crate::wifi_scan::wifi_scan;

use crate::runner::Runner;

use crate::options::structs::WifiConnectOptions;

impl Runner for WifiConnectOptions {
    fn run(&self) -> Result<(), RuwiError> {
        if let Some(essid) = self.get_given_essid() {
            let selected_network = get_network_from_given_essid(self, &essid)?;
            self.password_asker(&selected_network)
        } else {
            self.data_gatherer()
        }
    }
}

impl WifiConnectOptions {
    // TODO: decide if there should be an explicit service management step,
    //       or if services should be managed as they are used for scan/connect/etc
    //       Should you use the service of connect_via? of scan?
    //       It is probably best to have a utility function to start a given service, then
    //       run that as needed whenever a service might be needed.
    fn data_gatherer(&self) -> Result<(), RuwiError> {
        let (known_network_names, scan_result) = gather_wifi_network_data(self, None)?;
        self.network_parser_and_annotator(&known_network_names, &scan_result)
    }

    fn network_parser_and_annotator(
        &self,
        known_network_names: &KnownNetworkNames,
        scan_result: &ScanResult,
    ) -> Result<(), RuwiError> {
        let parse_results = parse_result(self, &scan_result)?;
        let annotated_networks =
            annotate_networks(self, &parse_results.seen_networks, &known_network_names);
        if should_retry_with_synchronous_scan(self, &annotated_networks) {
            self.synchronous_rescan(SynchronousRescanType::Automatic)
        } else {
            self.network_sorter(annotated_networks)
        }
    }

    fn synchronous_rescan(&self, rescan_type: SynchronousRescanType) -> Result<(), RuwiError> {
        let (known_network_names, scan_result) = gather_wifi_network_data(self, Some(rescan_type))?;
        self.network_parser_and_annotator(&known_network_names, &scan_result)
    }

    fn network_sorter(&self, annotated_networks: AnnotatedNetworks) -> Result<(), RuwiError> {
        let sorted_networks = sort_and_filter_networks(self, annotated_networks);
        self.network_selector(&sorted_networks)
    }

    fn network_selector(&self, sorted_networks: &SortedUniqueNetworks) -> Result<(), RuwiError> {
        match select_network(self, sorted_networks) {
            Ok(selected_network) => self.password_asker(&selected_network),
            Err(err) => match &err.kind {
                RuwiErrorKind::RefreshRequested => {
                    self.synchronous_rescan(SynchronousRescanType::ManuallyRequested)
                }
                _ => Err(err),
            },
        }
    }

    fn password_asker(&self, selected_network: &AnnotatedWirelessNetwork) -> Result<(), RuwiError> {
        let maybe_key = possibly_get_encryption_key(self, selected_network)?;
        self.network_configurator_and_connector(&selected_network, &maybe_key)
    }

    fn network_configurator_and_connector(
        &self,
        selected_network: &AnnotatedWirelessNetwork,
        maybe_key: &Option<String>,
    ) -> Result<(), RuwiError> {
        possibly_configure_network(self, &selected_network, &maybe_key)?;
        connect_to_network(self, &selected_network, &maybe_key)?;
        // TODO: Test connection here
        Ok(())
    }
}

fn get_network_from_given_essid<O>(
    options: &O,
    essid: &str,
) -> Result<AnnotatedWirelessNetwork, RuwiError>
where
    O: Global + Wifi + WifiConnect + LinuxNetworkingInterface,
{
    let is_known = find_known_network_names(options)?.contains(essid);
    let is_encrypted = options.get_given_encryption_key().is_some();
    Ok(AnnotatedWirelessNetwork::from_essid(
        essid.into(),
        is_known,
        is_encrypted,
    ))
}

fn gather_wifi_network_data<O>(
    options: &O,
    synchronous_rescan: Option<SynchronousRescanType>,
) -> Result<(KnownNetworkNames, ScanResult), RuwiError>
where
    O: 'static + Global + Wifi + WifiConnect + LinuxNetworkingInterface + Send + Sync + Clone,
{
    let options: &'static O = Box::leak(Box::new(options.clone()));

    let get_nw_names = thread::spawn(move || find_known_network_names(options));
    let get_scan_results = thread::spawn(move || wifi_scan(options, &synchronous_rescan));

    let known_network_names = await_thread(get_nw_names)??;
    let scan_result = await_thread(get_scan_results)??;

    Ok((known_network_names, scan_result))
}

#[inline]
fn await_thread<T>(handle: thread::JoinHandle<T>) -> Result<T, RuwiError> {
    handle.join().or_else(|_| {
        Err(rerr!(
            RuwiErrorKind::FailedToSpawnThread,
            "Failed to spawn thread."
        ))
    })
}

#[cfg(test)]
mod tests {}
