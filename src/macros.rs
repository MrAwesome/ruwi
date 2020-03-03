// TODO: see if it's possible to check if String::from should be run here
// TODO: add an optional stderr/etc message? an "extra data" blob?
#[macro_export(local_inner_macros)]
macro_rules! rerr {
    ( $kind:expr, $desc:expr $(,)? ) => {
        RuwiError {
            kind: $kind,
            desc: String::from($desc),
            extra_data: None,
        }
    };
    ( $kind:expr, $desc:expr, $($tag:expr => $data:expr),* ) => {{
        use std::collections::HashMap;
        let mut data = HashMap::new();
        $(data.insert($tag.to_string(), $data.to_string());)*
        RuwiError {
            kind: $kind,
            desc: String::from($desc),
            extra_data: Some(data),
        }
    }};
}

//#[macro_export(local_inner_macros)]
//macro_rules! optdbg {
//    ($y:expr, $($x:expr$(,)?)*) => {
//        if $y.d() {
//            std::dbg![$($x),*];
//        }
//    }
//}
