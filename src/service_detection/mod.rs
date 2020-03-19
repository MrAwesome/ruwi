mod check_mappings;
mod system_checker_real;

#[cfg(test)]
use mockall::{automock, predicate::*};

use std::fmt::{Debug, Display};

pub(crate) use system_checker_real::SystemCheckerReal;

pub(crate) trait HasSystemCheckMapping
where
    Self: Sized,
{
    fn get_system_check_mapping() -> Vec<(SystemCheckPredicate, Self)>;

    fn choose_best_from_system<S>(checker: &S, name: &str) -> Self
    where
        S: SystemChecksImpl,
        Self: Default + HasSystemCheckMapping + Display,
    {
        #[cfg(not(test))]
        return choose_best_from_system_impl(checker, name);
        #[cfg(test)]
        return choose_best_from_system_test_blockade(checker, name);
    }
}

fn choose_best_from_system_impl<S, T>(checker: &S, name: &str) -> T
where
    S: SystemChecksImpl,
    T: Default + HasSystemCheckMapping + Display,
{
    eprintln!("[NOTE]: No value was explicitly given for \"{}\", will check the system to determine the best value.", name);
    for (predicate, val) in T::get_system_check_mapping() {
        if check_predicate(checker, &predicate) {
            eprintln!(
                "[NOTE]: System check \"{}\" passed, will use \"{}\".",
                predicate, val
            );
            return val;
        }
    }

    let def = T::default();
    eprintln!(
        "[NOTE]: No system checks passed, will use the default value \"{}\".",
        def
    );
    def
}

#[cfg(test)]
fn choose_best_from_system_test_blockade<S, T>(_checker: &S, name: &str) -> T
where
    S: SystemChecksImpl,
    T: Default + HasSystemCheckMapping + Display,
{
    dbg!(name);
    let ret = T::default();
    eprintln!(
        "[NOTE]: Running system checks in test. Returning default value: {}",
        ret
    );
    ret
}

#[cfg_attr(test, automock)]
pub(crate) trait SystemChecksImpl {
    fn check_networkmanager_running(&self) -> bool;
    fn check_netctl_running(&self) -> bool;
    fn check_netctl_installed(&self) -> bool;
    fn check_networkmanager_installed(&self) -> bool;
    fn check_dhclient_installed(&self) -> bool;
    fn check_dhcpcd_installed(&self) -> bool;
}

use strum_macros::Display;
#[derive(Debug, Display)]
pub(crate) enum SystemCheckPredicate {
    NetworkManagerRunning,
    NetctlRunning,
    NetctlInstalled,
    NetworkManagerInstalled,
    DhclientInstalled,
    DhcpcdInstalled,
}

fn check_predicate<T: SystemChecksImpl>(checker: &T, check: &SystemCheckPredicate) -> bool {
    match check {
        SystemCheckPredicate::NetworkManagerRunning => checker.check_networkmanager_running(),
        SystemCheckPredicate::NetctlRunning => checker.check_netctl_running(),
        SystemCheckPredicate::NetctlInstalled => checker.check_netctl_installed(),
        SystemCheckPredicate::NetworkManagerInstalled => checker.check_networkmanager_installed(),
        SystemCheckPredicate::DhclientInstalled => checker.check_dhclient_installed(),
        SystemCheckPredicate::DhcpcdInstalled => checker.check_dhcpcd_installed(),
    }
}
