//! This is the main application

#![forbid(unsafe_code)]
#![allow(unused_imports)]
#![deny(unreachable_pub, private_in_public, unstable_features)]
#![warn(rust_2018_idioms, future_incompatible, nonstandard_style)]

use clap::Parser;
use lib::clap::{runner, Args, Implementation};
use lib::dispatch::*;

mod lib;

fn main() {
    let args = Args::parse();
    let _value: String = match args.implementation {
        Implementation::Default => runner(|| {
            println!("Running default");

            "default".to_string()
        }),
        Implementation::Dispatch => runner(|| {
            println!("Running dispatch");

            let x: Box<dyn AsRef<str>> = Box::new("hello".to_string());
            strlen_dyn2(x);

            // Use go-through pointer-indirection for something on the stack
            let x: &dyn AsRef<str> = &"hello".to_string();
            strlen_dyn(x);

            // Use our Hei trait
            let x: &dyn Hei = &"hei".to_string();
            x.weird();
            //x.need_sized();   // This is not object safe and therefore cannot be called on a trait-object
            say_hei(x);

            // Demonstrate that sized functions work just fine on any standard implementation of the trait
            let message = String::from("hello!");
            message.need_sized().to_string();

            let x: &dyn Hei = &"hei";
            x.weird();
            say_hei(x);

            "dispatch".to_string()
        }),
    };
    //assert_eq!(value, "hello".to_string());
}
