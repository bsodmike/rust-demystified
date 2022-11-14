use anyhow::{Error, Result};
use std::fmt::Display;

pub fn x(b: Box<impl Display + 'static>) -> Box<dyn Display> {
    b
}

pub fn runner() -> Result<()> {
    Ok(())
}
