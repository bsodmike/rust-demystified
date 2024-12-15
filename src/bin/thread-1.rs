#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Error;

const NUM_CPUS: u8 = 8;

pub fn run_sim() {
    let mut handles = Vec::with_capacity(NUM_CPUS as usize);

    let (tx, rx) = kanal::bounded::<()>(NUM_CPUS as usize);

    (0..NUM_CPUS - 1).for_each(|_| {
        let handle = std::thread::spawn(move || loop {

            //
        });

        handles.push(handle);
    })
}

pub fn main() -> Result<(), Error> {
    Ok(())
}
