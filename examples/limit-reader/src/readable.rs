use super::*;

pub trait ReaderTrait: std::io::Read {}
pub struct MyBufReader(pub BufReader<File>);

impl std::io::Read for MyBufReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl ReaderTrait for MyBufReader {}

pub trait Readable {
    // fn new(r: impl ReaderTrait) -> Self;

    fn perform_read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}

impl<R> Readable for LimitReader<R>
where
    R: ReaderTrait,
{
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
    pub fn new(r: R) -> Self {
        const LIMIT_READER: u64 = 8_u64;

        Self {
            reader: r,
            limit: LIMIT_READER as usize,
        }
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
