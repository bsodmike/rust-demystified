use anyhow::Result;
use std::fmt::Debug;
use std::future::Future;

pub(crate) mod disk;
pub(crate) mod s3;

pub trait Backend: Debug {
    fn persist(&mut self, data: &[u8]) -> impl Future<Output = Result<()>> + Send;
}
