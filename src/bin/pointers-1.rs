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

    let data = Box::new(0);
    let data_ptr = data.as_ref() as *const i32;
    tracing::info!(
        "Address of the Boxed object on the heap represented by data_ptr: {:p} / {:?}",
        data_ptr,
        data
    );

    let box_deref = unsafe { *data_ptr };
    tracing::info!(
        "We can defreference the pointer to the Boxed object on the heap: {:?}",
        box_deref
    );

    // Example with a String object
    let str_point_memory = String::from("Text on the Heap");
    let str_ref = &str_point_memory;
    tracing::info!(
        "Address of the object on the heap represented by str_point_memory: {:p}",
        str_point_memory.as_ptr(),
    );
    tracing::info!(
        "Address of str_point_memory on the stack {:p} / {:?}",
        str_ref,
        str_ref
    );

    Ok(())
}
