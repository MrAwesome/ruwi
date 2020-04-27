use crate::common::*;
use crate::options::wired::connect::*;
use crate::runner::Runner;
use crate::connect::raw_interface_connect::*;
use crate::interface_management::ip_interfaces::*;

impl Runner for WiredConnectOptions {
    fn run(&self) -> Result<(), RuwiError> {
        let interface = WiredIPInterface::from_name_or_first(self, self.get_given_interface_name())?;
        // TODO: 
        //  [] make configure network code, and netctl config writing, connection-type agnostic
        //  (impl NetctlConfigWriter<NetctlWifiConfig> for NetctlConfigHandler)
        // if in netctl mode:
            // check for given network identifier and matching connection type. 
            // if given, connect to it
            // if no network given, check system for connections if in netctl mode
            // if multiple connections detected, present selector
        let network = AnnotatedWiredNetwork::builder().build();

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

        RawInterfaceConnector::new(self, &interface).connect(network)?;
        
        // TODO: clean up
        println!("Successfully connected on \"{}\" using {}!", interface.get_ifname(), self.get_connect_via().to_string());
        Ok(())
    }
}
