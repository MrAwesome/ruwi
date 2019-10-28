use crate::structs::*;

pub fn connect_to_network(
    options: &Options,
    selected_network: &WirelessNetwork,
) -> ConnectionResult {
    // TODO: implement
    let res = ConnectionResult {
        connection_type: ConnectionType::Netctl,
        result: Ok("lawl".to_string()),
    };

    if options.debug {
        dbg!(&res);
    }

    res
}
