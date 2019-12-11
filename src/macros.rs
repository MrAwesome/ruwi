// TODO: see if it's possible to check if String::from should be run here
#[macro_export(local_inner_macros)]
macro_rules! rerr {
    ( $kind:expr, $desc:expr $(,)? ) => {{
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
