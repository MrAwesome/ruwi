use crate::cmdline_parser::get_options;
use crate::structs::*;
use std::rc::Rc;

//pub trait Runner {
//    fn run(&self) -> Option<Box<dyn Runner>>;
//}
//
//pub(crate) struct WifiScanner {
//    options: Rc<Options>,
//}
//
//// TODO: technical sheet out the flow. make unit tests!
//pub(crate) fn run_state_machine(options: Options) {
//    let mut next = Some(WifiScanner::new(Rc::new(options)));
//    while let Some(runner) = next {
//        next = runner.run();
//    }
//}
//
//impl WifiScanner {
//    fn new(options: Rc<Options>) -> Box<dyn Runner> {
//        Box::new(Self { options })
//    }
//}
//
//impl Runner for WifiScanner {
//    fn run(&self) -> Option<Box<dyn Runner>> {
//        let scan_result = ScanResult::default();
//        Some(WifiResultsParser::new(self.options.clone(), scan_result))
//    }
//}
//
//pub(crate) struct WifiResultsParser {
//    options: Rc<Options>,
//    scan_result: ScanResult,
//}
//
//impl WifiResultsParser {
//    fn new(options: Rc<Options>, scan_result: ScanResult) -> Box<dyn Runner> {
//        Box::new(Self {
//            options,
//            scan_result,
//        })
//    }
//}
//
//impl Runner for WifiResultsParser {
//    fn run(&self) -> Option<Box<dyn Runner>> {
//        dbg![&self.options, &self.scan_result];
//        None
//    }
//}

pub(crate) enum RuwiStep<'a> {
    Init,
    CmdLineParser,
    WifiInterfaceManager {
        options: &'a Options,
    },
    WifiServiceManager {
        options: &'a Options,
    },
    WifiDataGatherer {
        options: &'a Options,
    },
    WifiNetworkParserAndAnnotator {
        options: &'a Options,
        parse_result: ParseResult,
        known_network_names: KnownNetworkNames,
    },
    WifiNetworkSorter {
        options: &'a Options,
        seen_networks: AnnotatedNetworks,
    },
    WifiNetworkSelector {
        options: &'a Options,
        parse_result: ParseResult,
        known_network_names: KnownNetworkNames,
    },
    Test,
    Done
}

impl<'a> RuwiStep<'a> {
    fn run(&self) -> Result<Self, RuwiError> {
        match self {
            Self::CmdLineParser => {
                let options = &get_options()?;
                Ok(Self::WifiInterfaceManager { options })
            }
            Test => Done,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_runner_functionality() {
        // TODO: can there be an enum with all the runner types in it for matching against?
    }
}
