use super::*;
use crate::enums::*;

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

#[cfg(test)]
mod tests {
    use super::*;
    use paste;

    macro_rules! mock_func {
        ( $mock:ident, $times:expr, $funcname:ident, $retval:expr ) => {
            paste::expr! {
                 $mock.[<expect_ $funcname>] ()
                    .times($times)
                    .returning(|| $retval);
            }
        };
    }

    macro_rules! mock_func_not_called {
        ( $mock:ident, $funcname:ident ) => {
            paste::expr! {
                 $mock.[<expect_ $funcname>] ()
                    .times(0);
            }
        };
    }

    #[test]
    fn test_wificonn_default() {
        let mut mock = MockSystemChecksImpl::new();
        mock_func!(mock, 1, check_networkmanager_running, true);
        mock_func_not_called!(mock, check_netctl_running);
        mock_func_not_called!(mock, check_netctl_installed);
        mock_func_not_called!(mock, check_networkmanager_installed);
        assert_eq!(
            WifiConnectionType::Nmcli,
            choose_best_from_system_impl(&mock, "fake_name")
        );
    }

    #[test]
    fn test_wificonn_nwmgr_running() {
        let mut mock = MockSystemChecksImpl::new();
        mock_func!(mock, 1, check_networkmanager_running, false);
        mock_func!(mock, 1, check_netctl_running, false);
        mock_func!(mock, 1, check_netctl_installed, false);
        mock_func!(mock, 1, check_networkmanager_installed, false);
        assert_eq!(
            WifiConnectionType::default(),
            choose_best_from_system_impl(&mock, "fake_name")
        );
    }

    #[test]
    fn test_wificonn_netctl_installed() {
        let mut mock = MockSystemChecksImpl::new();
        mock_func!(mock, 1, check_networkmanager_running, false);
        mock_func!(mock, 1, check_netctl_running, false);
        mock_func!(mock, 1, check_netctl_installed, true);
        mock_func_not_called!(mock, check_networkmanager_installed);
        assert_eq!(
            WifiConnectionType::default(),
            choose_best_from_system_impl(&mock, "fake_name")
        );
    }

    #[test]
    fn test_wiredconn_dhcpcd_installed() {
        let mut mock = MockSystemChecksImpl::new();
        mock_func!(mock, 1, check_networkmanager_running, false);
        mock_func!(mock, 1, check_netctl_running, false);
        mock_func!(mock, 1, check_netctl_installed, false);
        mock_func!(mock, 1, check_networkmanager_installed, false);
        mock_func!(mock, 1, check_dhclient_installed, false);
        mock_func!(mock, 1, check_dhcpcd_installed, true);
        assert_eq!(
            WiredConnectionType::Dhcpcd,
            choose_best_from_system_impl(&mock, "fake_name")
        );
    }

    #[test]
    fn test_wiredconn_netctl_installed() {
        let mut mock = MockSystemChecksImpl::new();
        mock_func!(mock, 1, check_networkmanager_running, false);
        mock_func!(mock, 1, check_netctl_running, false);
        mock_func!(mock, 1, check_netctl_installed, true);
        mock_func_not_called!(mock, check_networkmanager_installed);
        mock_func_not_called!(mock, check_dhclient_installed);
        mock_func_not_called!(mock, check_dhcpcd_installed);
        assert_eq!(
            WiredConnectionType::Netctl,
            choose_best_from_system_impl(&mock, "fake_name")
        );
    }
}
