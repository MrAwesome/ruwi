use crate::errors::*;
use crate::options::interfaces::*;
use crate::options::wired::connect::*;
use crate::runner::Runner;
use crate::connect::raw_interface_connect::*;
use crate::interface_management::ip_interfaces::*;

impl Runner for WiredConnectOptions {
    fn run(&self) -> Result<(), RuwiError> {
        let interface = WiredIPInterface::from_name_or_first(self, self.get_given_interface_name())?;
        RawInterfaceConnector::new(self, &interface, self.get_connect_via()).connect()?;
        
        // TODO: clean up
        println!("Successfully connected on \"{}\" using {}!", interface.get_ifname(), self.get_connect_via().to_string());
        Ok(())
    }
}
