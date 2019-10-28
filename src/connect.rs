use crate::structs::*;

pub fn connect_to_network(options: Options, selected_network: WirelessNetwork) -> ConnectionResult {
    ConnectionResult {
        connection_type: ConnectionType::Netctl,
        result: Ok("lawl".to_string()),
    }
}
