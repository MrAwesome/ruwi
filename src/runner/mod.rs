use crate::errors::RuwiError;

mod wifi;
mod wired;
mod clear;

pub trait Runner {
    fn run(&self) -> Result<(), RuwiError>;
}
