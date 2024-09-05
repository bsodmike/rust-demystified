use limit_reader::prelude::*;

extern crate limit_reader;

fn main() -> anyhow::Result<()> {
    let mut object = Object::new();
    let _read_size = object.read("./source.txt".into(), false)?;

    Ok(())
}
