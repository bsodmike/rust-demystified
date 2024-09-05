use super::*;

pub struct MyBufReader<Z: Read>(pub Z);

impl<Z: Read> Read for MyBufReader<Z> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

pub trait Readable {
    fn perform_read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}

impl<R> Readable for LimitReaderPrivate<R>
where
    R: Read,
{
    fn perform_read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read(buf)
    }
}

pub(crate) struct LimitReaderPrivate<R>
where
    R: Read,
{
    reader: R,
    limit: usize,
}

impl<R> LimitReaderPrivate<R>
where
    R: Read,
{
    pub fn new(r: R, limit: usize) -> Self {
        Self { reader: r, limit }
    }
}

impl<R> Read for LimitReaderPrivate<R>
where
    R: Read,
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

impl<R> BufRead for LimitReaderPrivate<R>
where
    R: Read,
{
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        unimplemented!("LimitReaderPrivate should never call `fill_buf`")
    }

    fn consume(&mut self, _: usize) {}
}
