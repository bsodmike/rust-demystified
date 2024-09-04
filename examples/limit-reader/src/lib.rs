#![allow(unused_imports, dead_code, unused_variables)]

use anyhow::{Context, Result};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use readable::LimitReader;
use readable::MyBufReader;
use readable::Readable;
use std::any;
use std::ffi::CStr;
use std::fmt;
use std::fmt::format;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

const BUF_SIZE: usize = 1024;

pub(crate) mod readable;

/// Re-exports
pub mod prelude {
    pub use crate::Object;
    pub use anyhow::{Context, Result};
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    Blob,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Blob => write!(f, "blob"),
        }
    }
}

pub struct Object<R> {
    pub(crate) kind: Kind,
    pub(crate) expected_size: u64,
    pub(crate) reader: R,
}

impl<R> fmt::Display for Object<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "object")
    }
}

impl Object<()> {
    pub fn read(source: PathBuf, mut buf: [u8; BUF_SIZE]) -> Result<usize> {
        let f = std::fs::File::open(source).context("Unable to open provided path")?;
        // let z = ZlibDecoder::new(f);
        let z = MyBufReader(BufReader::new(f));

        let mut reader = LimitReader::new(z);

        let try_read = reader.perform_read(&mut buf);
        match try_read {
            Ok(value) => Ok(value),
            Err(err) => {
                let detail = err.to_string();
                return Err(err)
                    .context(format!("LimitReader failed to read the thing: {}", detail));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use readable::LimitReader;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn limit_reader_should_error() {
        let dir = tempdir().unwrap();

        let file_path = dir.path().join("test-source-data.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Mike was here. Briefly.").unwrap();

        let buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        if let Err(err) = Object::<()>::read(file_path, buf) {
            assert_eq!(
                "LimitReader failed to read the thing: too many bytes",
                err.to_string()
            )
        }

        drop(file);
        dir.close().unwrap();
    }
}
