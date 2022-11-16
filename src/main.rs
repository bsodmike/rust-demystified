//! This is the main application

// Disabled for use of UnsafeCell
//#![forbid(unsafe_code)]
#![allow(unused_imports)]
#![deny(unreachable_pub, private_in_public, unstable_features)]
#![warn(rust_2018_idioms, future_incompatible, nonstandard_style)]

use anyhow::{Error, Result};
use clap::Parser;
use lib::cli::{runner, Args, Commands};
use lib::{builder::TaskManagerBuilder, dispatch::*, oop_pattern::*, smart_pointers::*, traits};
use log::{debug, info};
use std::io::Read;
use std::sync::Arc;

pub mod lib;
// pub mod sandbox;

fn main() -> Result<()> {
    env_logger::init();

    let cli = Args::parse();

    match &cli.command {
        Some(Commands::Dispatch) => runner(|| {
            info!("Tutorial: Dynamic dispatch\n");

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

            Ok(())
        })?,
        Some(Commands::Builder) => runner(|| {
            info!("Tutorial: Builder pattern\n");

            let task_manager = TaskManagerBuilder::new().count(10).build();

            debug!("Task manager.count: {}", task_manager.count());
            assert_eq!(*task_manager.count(), 10);

            Ok(())
        })?,
        Some(Commands::TypeState) => {
            runner(|| {
                info!("Tutorial: OOP design pattern with Type State\n");

                let mut post = Post::new();
                post.add_text("I ate a salad for lunch today");

                let post = post.request_review();
                assert_eq!("I ate a salad for lunch today", post.review());

                let mut post = post.reject("Salad isn't available today");
                assert_eq!("Make changes to 'I ate a salad for lunch today' as Salad isn't available today", post.get_feedback());

                let post = post.replace_text("I ate fish at lunch");

                let post = post.approve();
                assert_eq!("I ate fish at lunch", post.content());
                Ok(())
            })?
        }
        Some(Commands::SmartPointers) => runner(|| {
            info!("Tutorial: Smart pointers\n");

            let new_message = MessageBuilder::new().content("hello").build();
            let new_message = new_message.update("foo");

            let byte_zero: u8 = 0;
            assert_ne!(new_message.bytes(), &vec![byte_zero]);

            let message = new_message.update("Häagen-Dazs");
            assert_eq!(
                message.content_from_bytes().unwrap(),
                "Häagen-Dazs".to_string()
            );

            // Example of using a new-type to implement the Into trait
            struct BytesToString {
                value: String,
            }

            impl BytesToString {
                pub(crate) fn new(value: &Vec<u8>) -> Self {
                    Self {
                        value: String::from_utf8(value.clone()).unwrap_or_default(),
                    }
                }
            }

            impl Into<String> for BytesToString {
                fn into(self) -> String {
                    self.value
                }
            }

            let x = Cell::new(message.bytes());

            // BytesToString is used as a new-type to convert Vec<u8> to a String
            let contents: String = BytesToString::new(x.get()).into();
            assert_eq!(contents, "Häagen-Dazs".to_string());

            Ok(())
        })?,
        Some(Commands::Traits) => runner(|| {
            info!("Tutorial: Traits\n");

            traits::runner()?;

            Ok(())
        })?,
        Some(Commands::Conversion) => runner(|| {
            info!("Tutorial: Conversion\n");

            lib::conversion::runner()?;

            Ok(())
        })?,
        _ => info!("Command not found"),
    };

    Ok(())
}
