//! Exposes [Object] which is a limit reader, that protects against zip-bombs and other nefarious activities.
#![allow(unused_imports, dead_code, unused_variables)]
#![warn(missing_docs)]

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

pub(crate) mod readable;

/// Re-exports
pub mod prelude {
    pub use crate::Object;
    pub use anyhow::{Context, Result};
}

/// [Object] record struct which holds a buffer for the limit reader.
pub struct Object {
    buf: [u8; Self::DEFAULT_BUF_SIZE],
    expected_size: u64,
}

// Holds a `LimitReader` with a default buffer of size `Object::DEFAULT_BUF_SIZE`
impl Object {
    /// Default buffer size for the internal `LimitReader`
    pub const DEFAULT_BUF_SIZE: usize = 1024;

    /// Create a new instance of [Object]
    pub fn new() -> Self {
        Self {
            buf: [0; Self::DEFAULT_BUF_SIZE],
            expected_size: (Self::DEFAULT_BUF_SIZE - 1) as u64,
        }
    }

    /// Increase the allowed limit on the `LimitReader`
    pub fn limit(&mut self, limit: u64) {
        self.expected_size = limit;
    }

    /// Read from provided source file.  If the source data is already Zlib compressed, optionally decode the data stream before reading it through a limit-reader.
    pub fn read(&mut self, source: PathBuf, decode_zlib: bool) -> Result<usize> {
        let f = std::fs::File::open(source).context("Unable to open provided path")?;
        if decode_zlib {
            let z = ZlibDecoder::new(f);
            let buf_reader = MyBufReader(z);
            let reader = LimitReader::new(buf_reader, self.expected_size as usize);

            self.try_read(reader)
        } else {
            let buf_reader = MyBufReader(BufReader::new(f));
            let reader = LimitReader::new(buf_reader, self.expected_size as usize);

            self.try_read(reader)
        }
    }

    fn try_read(&mut self, mut reader: impl Readable) -> Result<usize> {
        let try_read = reader.perform_read(&mut self.buf);
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
    use flate2::read;
    use readable::LimitReader;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn limit_reader_works() {
        let dir = tempdir().unwrap();

        let text = "Mike was here. Briefly.";
        let file_path = dir.path().join("test_output.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{}", &text).unwrap();

        let mut object = Object::new();

        match object.read(file_path, false) {
            Ok(read_size) => {
                assert!(read_size == 24);
            }
            Err(err) => unreachable!(),
        }

        let persisted_text = String::from_utf8(object.buf[..24].to_vec()).unwrap();
        assert_eq!(persisted_text, format!("{}\n", &text).to_string());

        drop(file);
        dir.close().unwrap();
    }

    #[test]
    fn limit_reader_should_error() {
        let dir = tempdir().unwrap();

        let text = "Mike was here. Briefly.";
        let file_path = dir.path().join("test_output.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{}", &text).unwrap();

        let mut object = Object::new();
        object.limit(8);

        match object.read(file_path, false) {
            Ok(read_size) => {
                assert!(read_size == 24);
            }
            Err(err) => {
                assert_eq!(
                    "LimitReader failed to read the thing: too many bytes",
                    err.to_string()
                );
            }
        }

        drop(file);
        dir.close().unwrap();
    }

    #[test]
    fn limit_reader_with_decode_zlib() {
        let dir = tempdir().unwrap();

        let text = "Mike was here. Briefly.";
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(text.as_bytes()).unwrap();
        let compressed = e.finish().unwrap();

        let file_path = dir.path().join("test_output.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(&compressed).unwrap();

        let mut object = Object::new();
        match object.read(file_path, true) {
            Ok(read_size) => {
                let persisted_text = String::from_utf8(object.buf[..read_size].to_vec()).unwrap();
                assert_eq!(persisted_text, format!("{}", &text).to_string());
            }
            Err(err) => unreachable!(),
        };

        drop(file);
        dir.close().unwrap();
    }

    #[test]
    fn limit_reader_with_decode_zlib_should_error() {
        let dir = tempdir().unwrap();

        let text = "Mike was here. Briefly.";
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(text.as_bytes()).unwrap();
        let compressed = e.finish().unwrap();

        let file_path = dir.path().join("test_output.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(&compressed).unwrap();

        let mut object = Object::new();

        // NOTE: This should error due to exceeding our limit.
        object.limit(8);

        match object.read(file_path, true) {
            Ok(read_size) => {
                let persisted_text = String::from_utf8(object.buf[..read_size].to_vec()).unwrap();
                assert_eq!(persisted_text, format!("{}", &text).to_string());
            }
            Err(err) => assert_eq!(
                "LimitReader failed to read the thing: too many bytes",
                err.to_string()
            ),
        };

        drop(file);
        dir.close().unwrap();
    }

    #[test]
    fn limit_reader_decode_zlib_error_on_corrupt_deflate_stream() {
        let dir = tempdir().unwrap();

        let text = "Mike was here. Briefly.";
        let file_path = dir.path().join("test_output.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{}", &text).unwrap();

        let mut object = Object::new();

        match object.read(file_path, true) {
            Ok(read_size) => {
                assert!(read_size == 24);
            }
            Err(err) => assert_eq!(
                "LimitReader failed to read the thing: corrupt deflate stream",
                err.to_string()
            ),
        }

        drop(file);
        dir.close().unwrap();
    }
}
