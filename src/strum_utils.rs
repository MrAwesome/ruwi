use clap::ArgMatches;
use std::str::FromStr;
use strum::AsStaticRef;
use strum::IntoEnumIterator;

pub(crate) fn possible_vals<'a, E, I>() -> Vec<&'static str>
where
    E: IntoEnumIterator<Iterator = I> + AsStaticRef<str>,
    I: Iterator<Item = E>,
{
    E::iter().map(|x| x.as_static()).collect::<Vec<_>>()
}

pub(crate) fn get_val<T: FromStr + Default>(m: &ArgMatches, arg: &str) -> T
where
    T::Err: std::fmt::Debug,
{
    let scan_type = match m.value_of(arg) {
        Some(x) => T::from_str(x).expect(&format!("Failed to parse: {}", arg)),
        None => T::default(),
    };
    scan_type
}
