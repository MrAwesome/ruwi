#[macro_export(local_inner_macros)]
macro_rules! rerr {
    ( $kind:expr, $desc:expr ) => {{
        use crate::structs::{RuwiError, RuwiErrorKind};
        RuwiError {
            kind: $kind,
            desc: String::from($desc),
        }
    }};
}

//#[macro_export(local_inner_macros)]
//macro_rules! optdbg {
//    ($($x:expr,)*) => (std::dbg![$($x),*]);
//}
