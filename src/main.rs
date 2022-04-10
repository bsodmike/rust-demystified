//! This is the main application

#![forbid(unsafe_code)]
#![allow(unused_imports)]
#![deny(unreachable_pub, private_in_public, unstable_features)]
#![warn(rust_2018_idioms, future_incompatible, nonstandard_style)]

use clap::Parser;
use lib::cli::{runner, Args, Commands};
use lib::dispatch::*;
use log::{debug, info};

mod lib;

fn main() {
    env_logger::init();

    let cli = Args::parse();

    match &cli.command {
        Some(Commands::Dispatch) => runner(|| {
            info!("Running dispatch");

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
        }),

        //Some(Commands::Default) => runner(|| {
        //    info!("Running default");
        //}),
        _ => info!("Command not found"),
    };
    //assert_eq!(value, "hello".to_string());
}
