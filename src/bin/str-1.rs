#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt::Debug;

use anyhow::Error;

extern crate tutorials;

pub fn main() -> Result<(), Error> {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "tutorials=info,tower_http=trace,tokio=trace,runtime=trace",
        )
    }
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_file(true)
        .with_line_number(true)
        .init();

    let mut box_string: Box<str> = String::from("hello").into_boxed_str();
    box_string.make_ascii_uppercase();
    tracing::info!("{:?}", box_string);

    let mut vec = Vec::with_capacity(10);
    vec.extend([1, 2, 3]);

    assert!(vec.capacity() >= 10);

    // NOTE: This erases the capacity of the vector
    let slice = vec.into_boxed_slice();

    let x = slice.iter().map(|el| *el).collect::<Vec<usize>>();
    dbg!(x);

    assert_eq!(slice.into_vec().capacity(), 3);

    Ok(())
}
