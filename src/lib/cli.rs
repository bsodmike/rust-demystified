use clap::{Parser, Subcommand};
use std::str::FromStr;

/// Program to run rust tutorials
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Tutorial on dynamic dispatch
    Dispatch,

    /// Builder pattern
    Builder,

    /// Type state
    TypeState,
}

pub(crate) fn runner<T>(mut mk: impl FnMut() -> T) -> T {
    mk()
}
