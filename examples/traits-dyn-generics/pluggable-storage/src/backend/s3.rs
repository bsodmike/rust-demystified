#[allow(unused_imports)]
use anyhow::{Context, Result};
use derive_builder::Builder;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use super::Backend;

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct S3Config {
  bucket: String,
  file_name: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct S3Backend {
  config: S3Config,
}

impl S3Backend {
  fn new(config: S3Config) -> Self {
    Self { config }
  }

  pub fn bucket(&self) -> String {
    self.config.bucket.to_string()
  }

  pub fn file_name(&self) -> String {
    self.config.file_name.to_string()
  }
}

pub trait S3BackendTrait {
  fn new(config: S3Config) -> Self;
}

impl S3BackendTrait for S3Backend {
  // NOTE: This is a provided method via the Trait `S3BackendTrait`
  fn new(config: S3Config) -> Self {
    // NOTE: this is an associated method on `S3Backend`
    Self::new(config)
  }
}

#[derive(Debug)]
pub struct S3BackendBuilder<T>(T);

// Example on implemeting Deref for an inner type.
impl<T> Deref for S3BackendBuilder<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T> DerefMut for S3BackendBuilder<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<T> S3BackendBuilder<T>
where
  T: S3BackendTrait,
{
  pub fn new(config: S3Config) -> Self {
    Self(T::new(config))
  }

  pub fn build(self) -> T {
    self.0
  }
}

impl Backend for S3Backend {
  async fn persist(&mut self, data: &[u8]) -> Result<()> {
    let bucket_name = &self.bucket();
    let file_name = &self.file_name();

    unimplemented!();

    // if let Err(err) = _ {
    //   return Err(err).with_context(|| format!("Failed to persist to {:?}", self.path));
    // }

    Ok(())
  }
}
