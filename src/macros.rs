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
        let data = std::vec![$(($tag.to_string(), $data.to_string())),*];
        RuwiError {
            kind: $kind,
            desc: String::from($desc),
            extra_data: Some(data),
        }
    }};
}

#[macro_export]
macro_rules! string_container {
    ( $($name:ident),* ) => {
        use std::{
            error::Error,
            fmt,
            string::ToString,
        };

        $(
        #[derive(Debug, Clone, Eq, PartialEq)]
        pub(crate) struct $name(String);

        impl $name {
            pub fn new(msg: impl ToString) -> Self {
                $name(msg.to_string())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.0.as_ref()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_ref())
            }
        }

        impl Error for $name {}

        impl From<String> for $name {
            fn from(string: String) -> $name {
                $name::new(string)
            }
        }

        impl From<&String> for $name {
            fn from(string: &String) -> $name {
                $name::new(string)
            }
        }
        impl From<&str> for $name {
            fn from(string: &str) -> $name {
                $name::new(string)
            }
        }

        )*
    }
}

//#[macro_export(local_inner_macros)]
//macro_rules! optdbg {
//    ($y:expr, $($x:expr$(,)?)*) => {
//        if $y.d() {
//            std::dbg![$($x),*];
//        }
//    }
//}
