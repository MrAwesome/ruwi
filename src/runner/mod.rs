// TODO: move these pieces into their own helper library
use crate::options::structs::*;
use crate::structs::*;

mod wifi;
use wifi::RunStart;

pub fn run_ruwi_using_state_machine(command: RuwiCommand) -> Result<(), RuwiError> {
    match &command {
        RuwiCommand::Wifi(RuwiWifiCommand::Connect(options)) => options.run(),

        RuwiCommand::Wired(RuwiWiredCommand::Connect) => unimplemented!(),
        RuwiCommand::Bluetooth(RuwiBluetoothCommand::Pair) => unimplemented!(),
    }
    //      RuwiCommand::BluetoothPair => {}
    //      RuwiCommand::WifiSelect => {}
    //      RuwiCommand::WifiEditNetwork => {}
    //      RuwiCommand::WifiDeleteNetwork => {}
    //      RuwiCommand::List => {}
    //      RuwiCommand::DumpJSON => {}
    //      RuwiCommand::Disconnect => {}
}
