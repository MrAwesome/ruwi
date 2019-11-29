#[macro_export(local_inner_macros)]
macro_rules! errbox {
    ( $x:expr ) => {{
        use std::error::Error;
        Box::<dyn Error + Send + Sync>::from($x)
    }};
}
