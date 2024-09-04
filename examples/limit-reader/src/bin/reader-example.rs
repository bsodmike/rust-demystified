use limit_reader::prelude::*;

extern crate limit_reader;

const BUF_SIZE: usize = 1024;

fn main() -> anyhow::Result<()> {
    let buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
    let _read_size = Object::read("./source.txt".into(), buf).unwrap();

    Ok(())
}
