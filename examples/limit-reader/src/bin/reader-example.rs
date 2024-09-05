use limit_reader::prelude::*;

extern crate limit_reader;

fn main() -> anyhow::Result<()> {
    let mut limit_reader = LimitReader::new();
    let _read_size = limit_reader.read("./source.txt".into(), false)?;

    Ok(())
}
