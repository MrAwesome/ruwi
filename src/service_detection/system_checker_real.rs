use crate::options::interfaces::Global;
use super::*;

pub(crate) struct SystemCheckerReal<'a, O: Global> {
    opts: &'a O
}


impl<'a, O: Global> SystemChecker<'a, O> for SystemCheckerReal<'a, O> {
    fn new(opts: &'a O) -> Self {
        Self {
            opts
        }
    }
}

impl<'a, O: Global> SystemChecksImpl<'a, O> for SystemCheckerReal<'a, O> {
    fn get_opts(&self) -> &'a O {
        self.opts
    }

    fn check_networkmanager_running(&self) -> bool {
        true
    }

    fn check_netctl_running(&self) -> bool {
        false
    }

    fn check_netctl_installed(&self) -> bool {
        false
    }

    fn check_networkmanager_installed(&self) -> bool {
        false
    }
}

