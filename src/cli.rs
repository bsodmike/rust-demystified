use anyhow::{Error, Result};
use clap::{Parser, Subcommand};
use std::str::FromStr;

/// Program to run rust tutorials
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Tutorial on dynamic dispatch
    Dispatch,

    /// Builder pattern
    Builder,

    /// Implementing an Object-Oriented Design Pattern with Type state
    TypeState,

    /// Smart pointers
    SmartPointers,

    /// Traits
    Traits,

    /// Conversion
    Conversion,

    /// Closures
    Closures,

    /// Challenge1
    Challenge1,

    /// PartialEq example
    PartialEq1,
}

pub fn runner<T>(mut mk: impl FnMut() -> Result<T>) -> Result<T> {
    mk()
}
