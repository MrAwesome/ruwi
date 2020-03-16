use crate::enums::*;
use super::*;

impl HasSystemCheckMapping for WifiConnectionType {
    fn get_system_check_mapping() -> Vec<(SystemCheck, Self)> {
        vec![
            (SystemCheck::NetworkManagerRunning, Self::Nmcli),
            (SystemCheck::NetctlRunning, Self::Netctl),
            (SystemCheck::NetctlInstalled, Self::Netctl),
            (SystemCheck::NetworkManagerInstalled, Self::Nmcli),
        ]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::system_checker_real::*;
    use crate::options::GlobalOptions;
    use crate::enums::WifiConnectionType;

    // TODO: this is where you'll want a mock object for systemchecker

    #[test]
    fn test_is_used()  {
        let opts = GlobalOptions::default();
        let checker = SystemCheckerReal::new(&opts);
        let wct = WifiConnectionType::choose_best_from_system(&checker);
        dbg!(wct);
        panic!();
    }
}
