use clap::Parser;
use std::str::FromStr;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub implementation: Implementation,
}

#[derive(Parser, Debug)]
pub enum Implementation {
    r#Default,
    Dispatch,
}

impl FromStr for Implementation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Self::Default),
            "dispatch" => Ok(Self::Dispatch),
            _ => Err(format!("unknown implementation {}", s)),
        }
    }
}

pub fn runner<T>(mut mk: impl FnMut() -> T) -> T {
    mk()
}
