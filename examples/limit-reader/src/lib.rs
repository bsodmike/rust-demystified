#![allow(unused_imports, dead_code, unused_variables)]

use anyhow::{Context, Result};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use readable::LimitReader;
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

pub(crate) struct Object<R> {
    pub(crate) kind: Kind,
    pub(crate) expected_size: u64,
    pub(crate) reader: R,
}

pub trait ReaderTrait: std::io::Read {}
struct MyBufReader(BufReader<File>);

impl std::io::Read for MyBufReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl ReaderTrait for MyBufReader {}

impl<R> Object<R>
where
    R: Readable + std::io::Read,
{
    pub(crate) fn read(source: PathBuf, mut buf: [u8; BUF_SIZE]) -> Result<usize> {
        let f = std::fs::File::open(source).context("Unable to open provided path")?;
        // let z = ZlibDecoder::new(f);
        let z = MyBufReader(BufReader::new(f));

        let mut reader = R::new(z);

        // NOTE old impl. Replacing this with generic type `R`
        // let mut reader = LimitReader {
        //     reader: z,
        //     limit: LIMIT_READER as usize,
        // };

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
        if let Err(err) = Object::<LimitReader<MyBufReader>>::read(file_path, buf) {
            assert_eq!(
                "LimitReader failed to read the thing: too many bytes",
                err.to_string()
            )
        }

        drop(file);
        dir.close().unwrap();
    }
}

pub(crate) mod readable {
    use super::*;

    pub trait Readable {
        fn new(r: impl ReaderTrait) -> Self;

        fn perform_read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
    }

    impl<R> Readable for LimitReader<R>
    where
        R: ReaderTrait,
    {
        // NOTE: This is a provided method via the Trait `Readable`
        fn new(r: impl ReaderTrait) -> Self {
            // NOTE: this is an associated method on `LimitReader<R>`
            Self::new(r)
        }

        // FIXME:
        // But LimitReader expects only the type that it contains, it can't support being created from any possible type implementing ReaderTrait
        //
        //     error[E0308]: mismatched types
        //     --> src/lib.rs:127:23
        //      |
        //  120 |     impl<R> Readable for LimitReader<R>
        //      |          - expected type parameter
        //  ...
        //  125 |         fn new(r: impl ReaderTrait) -> Self {
        //      |                   ---------------- found type parameter
        //  126 |             // NOTE: this is an associated method on `LimitReader<R>`
        //  127 |             Self::new(r)
        //      |             --------- ^ expected type parameter `R`, found type parameter `impl ReaderTrait`
        //      |             |
        //      |             arguments to this function are incorrect
        //      |
        //      = note: expected type parameter `R`
        //                 found type parameter `impl ReaderTrait`
        //      = note: a type parameter was expected, but a different one was found; you might be missing a type parameter or trait bound
        //      = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
        //  note: associated function defined here
        //     --> src/lib.rs:147:12
        //      |
        //  147 |         fn new(r: R) -> Self {
        //      |            ^^^ ----

        fn perform_read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.read(buf)
        }
    }

    pub(crate) struct LimitReader<R>
    where
        R: ReaderTrait,
    {
        pub reader: R,
        pub limit: usize,
    }

    impl<R> LimitReader<R>
    where
        R: ReaderTrait,
    {
        fn new(r: R) -> Self {
            const LIMIT_READER: u64 = 8_u64;

            Self {
                reader: r,
                limit: LIMIT_READER as usize,
            }

            // FIXME: ideas?
            // error[E0308]: mismatched types
            //    --> src/lib.rs:150:25
            //     |
            // 142 |     impl<R> LimitReader<R>
            //     |          - expected type parameter
            // ...
            // 146 |         fn new(r: impl ReaderTrait) -> Self {
            //     |                   ---------------- found type parameter
            // ...
            // 150 |                 reader: r,
            //     |                         ^ expected type parameter `R`, found type parameter `impl ReaderTrait`
            //     |
            //     = note: expected type parameter `R`
            //                found type parameter `impl ReaderTrait`
            //     = note: a type parameter was expected, but a different one was found; you might be missing a type parameter or trait bound
            //     = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
        }
    }

    impl<R> Read for LimitReader<R>
    where
        R: ReaderTrait,
    {
        fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
            if buf.len() > self.limit {}
            buf = &mut buf[..self.limit + 1];
            let n = self.reader.read(buf)?;
            if n > self.limit {
                return Err(io::Error::new(io::ErrorKind::Other, "too many bytes"));
            }
            self.limit -= n;
            Ok(n)
        }
    }

    impl<R> BufRead for LimitReader<R>
    where
        R: ReaderTrait,
    {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            unimplemented!("LimitReader should never call `fill_buf`")
        }

        fn consume(&mut self, amt: usize) {}
    }
}
