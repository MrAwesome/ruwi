mod check_mappings;
mod system_checker_real;

#[cfg(test)]
use mockall::{automock, predicate::*};

pub(crate) use system_checker_real::SystemCheckerReal;

pub(crate) trait HasSystemCheckMapping
where
    Self: Sized,
{
    fn get_system_check_mapping() -> Vec<(SystemCheckPredicate, Self)>;

    fn choose_best_from_system<S>(checker: &S) -> Self
    where
        S: SystemChecksImpl,
        Self: Default + HasSystemCheckMapping,
    {
        for (predicate, val) in Self::get_system_check_mapping() {
            if check_predicate(checker, &predicate) {
                return val;
            }
        }
        Self::default()
    }
}

#[cfg_attr(test, automock)]
pub(crate) trait SystemChecksImpl {
    fn check_networkmanager_running(&self) -> bool;
    fn check_netctl_running(&self) -> bool;
    fn check_netctl_installed(&self) -> bool;
    fn check_networkmanager_installed(&self) -> bool;
}

#[derive(Debug)]
pub(crate) enum SystemCheckPredicate {
    NetworkManagerRunning,
    NetctlRunning,
    NetctlInstalled,
    NetworkManagerInstalled,
}

fn check_predicate<T: SystemChecksImpl>(checker: &T, check: &SystemCheckPredicate) -> bool {
    match check {
        SystemCheckPredicate::NetworkManagerRunning => checker.check_networkmanager_running(),
        SystemCheckPredicate::NetctlRunning => checker.check_netctl_running(),
        SystemCheckPredicate::NetctlInstalled => checker.check_netctl_installed(),
        SystemCheckPredicate::NetworkManagerInstalled => checker.check_networkmanager_installed(),
    }
}
