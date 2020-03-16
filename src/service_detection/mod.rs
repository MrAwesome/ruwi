mod check_mappings;
mod system_checker_real;

use crate::options::interfaces::Global;

// if connection type isn't given:
//    check NetworkingServices installed/running, pick a WifiConnectionType
//    check NetworkingServices installed/running, pick a WiredConnectionType
//
// if scanning type isn't given:
//    check NetworkingServices installed/running, check scanning binaries installed/running, pick a WifiScanType

trait HasSystemCheckMapping {
    fn get_system_check_mapping() -> Vec<(SystemCheck, Self)> where Self: Sized;

    fn choose_best_from_system<'a, O, S>(checker: &S) -> Self 
        where 
            S: SystemChecker<'a, O> + SystemChecksImpl<'a, O>,
            O: Global + 'a,
            Self: 'a + Default,
    {
        
        for (predicate, val) in Self::get_system_check_mapping() {
            if checker.run_check(&predicate) {
                return val;
            }
        }
        Self::default()
    }
}

trait SystemChecker<'a, O: Global> {
    fn new(opts: &'a O) -> Self;

    fn run_check(&self, check: &SystemCheck) -> bool 
        where Self: SystemChecksImpl<'a, O>
    {
        match check {
            SystemCheck::NetworkManagerRunning => self.check_networkmanager_running(),
            SystemCheck::NetctlRunning => self.check_netctl_running(),
            SystemCheck::NetctlInstalled => self.check_netctl_installed(),
            SystemCheck::NetworkManagerInstalled => self.check_networkmanager_installed(),
        }
    }
}

trait SystemChecksImpl<'a, O: Global> {
    fn get_opts(&self) -> &'a O;
    fn check_networkmanager_running(&self) -> bool;
    fn check_netctl_running(&self) -> bool;
    fn check_netctl_installed(&self) -> bool;
    fn check_networkmanager_installed(&self) -> bool;
}

#[derive(Debug)]
pub(crate) enum SystemCheck {
    NetworkManagerRunning,
    NetctlRunning,
    NetctlInstalled,
    NetworkManagerInstalled,
}
