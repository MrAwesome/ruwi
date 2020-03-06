use crate::errors::*;

mod wifi;
mod wired;

pub trait Runner {
    fn run(&self) -> Result<(), RuwiError>;
}
