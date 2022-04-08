#![warn(rust_2018_idioms)]

use clap::Parser;
use lib::dispatch::{runner, Args, Implementation};

mod lib;

fn main() {
    let args = Args::parse();
    let value: String = match args.implementation {
        Implementation::Default => runner(|| {
            println!("Running default");

            String::from("hello")
        }),
    };
    assert_eq!(value, "hello".to_string());
}
