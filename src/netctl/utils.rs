use crate::common::*;
use crate::run_commands::SystemCommandRunner;

#[derive(Debug)]
pub(crate) struct NetctlIdentifier(String);

impl NetctlIdentifier {
    fn as_str(&self) -> &str {
        &self.0
    }
}

pub(crate) fn get_netctl_identifier(selected_network: &AnnotatedWirelessNetwork) -> Result<NetctlIdentifier, RuwiError> {
    let maybe_service_id = selected_network.get_service_identifier();
    if let Some(service_id) = maybe_service_id {
        match service_id {
            NetworkServiceIdentifier::Netctl(file_name) => Ok(NetctlIdentifier(file_name.to_string())),
            _ => Err(rerr!(
                RuwiErrorKind::InvalidServiceIdentifierType, 
                "Invalid service identifier type for ID {}! This usually indicates a breaking code change, and should be reported to the authors."
            ))
        }
    } else {
        Ok(NetctlIdentifier(selected_network.get_public_name().replace(" ", "_")))
    }
}


pub(crate) fn netctl_switch_to<O>(options: &O, netctl_identifier: NetctlIdentifier) -> Result<(), RuwiError> 
where O: Global
{
    SystemCommandRunner::new(options, "netctl", &["switch-to", netctl_identifier.as_str()])
        .run_command_pass(
            RuwiErrorKind::FailedToConnectViaNetctl,
            &format!("Failed to connect to netctl profile \"{}\"!", netctl_identifier.as_str()),
        )
}
