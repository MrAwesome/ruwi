use crate::common::*;
use crate::connect::raw_interface_connect::*;
use crate::interface_management::ip_interfaces::*;
use crate::netctl::NetctlConfigHandler;
use crate::options::wired::connect::*;
use crate::runner::Runner;

impl Runner for WiredConnectOptions {
    fn run(&self) -> Result<(), RuwiError> {
        // TODO: get all wired interfaces, and if in netctl mode, all netctl profiles.
        // If there's only one interface, just use it.
        // If there's only one netctl profile, use it. If a netctl profile refers to the seen
        // interface, prefer it.
        let interface =
            WiredIPInterface::from_name_or_first(self, self.get_given_interface_name())?;

        let networks = match self.get_given_profile_name() {
            Some(ident) => {
                eprintln!("[NOTE]: Using manually-specified netctl profile \"{}\".", ident);
                vec![AnnotatedWiredNetwork::builder()
                    .interface(interface.clone())
                    .service_identifier(NetworkServiceIdentifier::Netctl(ident.clone()))
                    .build()]
            },
            None => {
                match self.get_connect_via() {
                    WiredConnectionType::Netctl => get_netctl_wired_networks(self, &interface)?,
                    _ => vec![nw_from_interface(&interface)],
                }
            },
        };


        if networks.len() > 1 {
            eprintln!("[NOTE]: More than one matching netctl profile was found for interface {}. Will use the first. Manually specify the profile you want with `-p <profilename>` if this is not what you want.", interface.get_ifname());
        };

        let network = if networks.len() >= 1 {
            networks.first().unwrap()
        } else {
            unreachable!("We should have ensured by now that a network exists. If you see this, report a bug!")
        };

        if let None = network.get_service_identifier() {
            if let WiredConnectionType::Netctl = self.get_connect_via() {
                let handler = NetctlConfigHandler::new(self);
                handler.write_wired_config(&interface, &network)?;
            }
        };

        RawInterfaceConnector::new(self, &interface).connect(network)?;

        println!(
            "Successfully connected on \"{}\" using {}!",
            interface.get_ifname(),
            self.get_connect_via().to_string()
        );
        Ok(())

        // TODO:
        //  [] make configure network code, and netctl config writing, connection-type agnostic
        //  (impl NetctlConfigWriter<NetctlWifiConfig> for NetctlConfigHandler)
        // if in netctl mode:
        // check for given network identifier and matching connection type.
        // if given, connect to it
        // if no network given, check system for connections if in netctl mode
        // if multiple connections detected, present selector
        //
        //        let given_identifier = network.get_service_identifier();
        //
        //        let identifier = match given_identifier {
        //            Some(NetworkServiceIdentifier::Netctl(ident)) => {
        //                eprintln!("[NOTE]: Using manually-specified netctl profile \"{}\".", ident);
        //                NetctlIdentifier::from(ident.as_ref())
        //            }
        //
        //            None => {
        //                let handler = NetctlConfigHandler::new(self.options);
        //                let identifiers = handler.get_wired_configs_with_interface(ifname)?;
        //                // TODO: use selection here if multiple profiles detected?
        //                if identifiers.len() > 1 {
        //                    eprintln!("[NOTE]: More than one matching netctl profile was found for interface {}. Will use the first. Manually specify the profile you want with `-p <profilename>` if this is not what you want.", ifname);
        //                }
        //
        //                match identifiers.first() {
        //                    Some(identifier) => {
        //                        eprintln!("[NOTE]: Using existing netctl profile \"{}\".", identifier);
        //                        identifier.clone()
        //                    }
        //                    None => {
        //                        eprintln!("[NOTE]: No existing netctl profile found for interface {}. Will create one now.", ifname);
        //                        handler.write_wired_config(self.interface, &network)?
        //                    } //todo!("create the config and return its identifier (maybe check a flag for if we should?)"),
        //                }
        //            }
        //
    }
}

fn nw_from_interface(iface: &WiredIPInterface) -> AnnotatedWiredNetwork {
    AnnotatedWiredNetwork::builder()
        .interface(iface.clone())
        .build()
}

fn get_netctl_wired_networks<O: Global>(options: &O, interface: &WiredIPInterface) -> Result<Vec<AnnotatedWiredNetwork>, RuwiError> {
    let configs = NetctlConfigHandler::new(options).get_wired_configs(interface.get_ifname())?;
    let networks: Vec<AnnotatedWiredNetwork> = configs.iter().map(From::from).collect();

    let networks = if networks.len() < 1 {
        vec![nw_from_interface(interface)]
    } else {
        networks
    };

    Ok(networks)
}
