#![warn(rust_2018_idioms)]

use clap::Parser;
use lib::clap::{runner, Args, Implementation};
use lib::dispatch::*;

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

    // Dispatch
    let x: Box<dyn AsRef<str>> = Box::new("hello".to_string());
    strlen_dyn2(x);

    // Use go-through pointer-indirection for something on the stack
    let x: &dyn AsRef<str> = &"hello".to_string();
    strlen_dyn(x);
}
