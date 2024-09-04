use anyhow::{Context, Result};
use std::fmt::Debug;
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

pub(crate) mod disk;
pub(crate) mod s3;

pub trait Backend: Debug {
  fn persist(&mut self, data: &[u8]) -> impl Future<Output = Result<()>> + Send;
}
