use super::{HasSystemCheckMapping, SystemCheckPredicate, WifiConnectionType, WifiScanType, WiredConnectionType};

// if connection type isn't given:
//    check NetworkingServices installed/running, pick a WifiConnectionType
//    check NetworkingServices installed/running, pick a WiredConnectionType
//
// if scanning type isn't given:
//    check NetworkingServices installed/running, check scanning binaries installed/running, pick a WifiScanType

impl HasSystemCheckMapping for WiredConnectionType {
    fn get_system_check_mapping() -> Vec<(SystemCheckPredicate, Self)> {
        vec![
            (SystemCheckPredicate::NetworkManagerRunning, Self::Nmcli),
            (SystemCheckPredicate::NetctlRunning, Self::Netctl),
            (SystemCheckPredicate::NetctlInstalled, Self::Netctl),
            (SystemCheckPredicate::NetworkManagerInstalled, Self::Nmcli),
            (SystemCheckPredicate::DhclientInstalled, Self::Dhclient),
            (SystemCheckPredicate::DhcpcdInstalled, Self::Dhcpcd),
        ]
    }
}

impl HasSystemCheckMapping for WifiConnectionType {
    fn get_system_check_mapping() -> Vec<(SystemCheckPredicate, Self)> {
        vec![
            (SystemCheckPredicate::NetworkManagerRunning, Self::Nmcli),
            (SystemCheckPredicate::NetctlRunning, Self::Netctl),
            (SystemCheckPredicate::NetctlInstalled, Self::Netctl),
            (SystemCheckPredicate::NetworkManagerInstalled, Self::Nmcli),
        ]
    }
}

impl HasSystemCheckMapping for WifiScanType {
    fn get_system_check_mapping() -> Vec<(SystemCheckPredicate, Self)> {
        vec![(SystemCheckPredicate::NetworkManagerRunning, Self::Nmcli)]
    }
}
