use clap::ArgMatches;
use std::str::FromStr;
use strum::AsStaticRef;
use strum::IntoEnumIterator;

pub(crate) fn possible_string_vals<E, I>() -> Vec<&'static str>
where
    E: IntoEnumIterator<Iterator = I> + AsStaticRef<str>,
    I: Iterator<Item = E>,
{
    E::iter().map(|x| x.as_static()).collect::<Vec<_>>()
}

pub(crate) fn get_val_as_enum<T>(m: &ArgMatches, arg: &str) -> T
where
    T: FromStr + Default,
    T::Err: std::fmt::Debug,
{
    match m.value_of(arg) {
        Some(x) => parse_as_enum(x),
        None => T::default(),
    }
}

pub(crate) fn parse_as_enum<T>(x: &str) -> T
where
    T: FromStr,
{
    T::from_str(x).unwrap_or_else(|_| panic!(format!("Failed to parse: {}", x)))
}
