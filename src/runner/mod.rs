use crate::errors::*;

mod wifi;

pub trait Runner {
    fn run(&self) -> Result<(), RuwiError>;
}
