use anyhow::{Context, Result};
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

use super::Backend;

#[derive(Debug)]
pub struct FileBackend {
  path: PathBuf,
}

impl FileBackend {
  fn new(s: &str) -> Self {
    Self { path: s.into() }
  }
}

pub trait FileBackendTrait {
  fn new(s: &str) -> Self;
}

impl FileBackendTrait for FileBackend {
  // NOTE: This is a provided method via the Trait `FileBackendTrait`
  fn new(s: &str) -> Self {
    // NOTE: this is an associated method on `FileBackend`
    Self::new(s)
  }
}

#[derive(Debug)]
pub struct FileBackendBuilder<T>(T);

// Example on implemeting Deref for an inner type.
impl<T> Deref for FileBackendBuilder<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T> DerefMut for FileBackendBuilder<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<T> FileBackendBuilder<T>
where
  T: FileBackendTrait,
{
  pub fn new(path: &str) -> Self {
    Self(T::new(path))
  }

  pub fn build(self) -> T {
    self.0
  }
}

impl Backend for FileBackend {
  async fn persist(&mut self, data: &[u8]) -> Result<()> {
    if let Err(err) = tokio::fs::write(&self.path, data).await {
      return Err(err).with_context(|| format!("Failed to persist to {:?}", self.path));
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[should_panic]
  #[tokio::test]
  async fn panic_when_writing_to_missing_directory() {
    let data = String::from("If my calculations are correct, when this baby hits eighty-eight miles per hour... you're gonna see some serious s**t.");
    let backend_builder: FileBackendBuilder<FileBackend> = FileBackendBuilder::new("./missing/output.json");
    let backend = &mut backend_builder.build();

    backend.persist(data.as_bytes()).await.unwrap();
  }
}
