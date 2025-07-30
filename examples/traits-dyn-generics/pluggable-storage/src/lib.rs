pub(crate) mod backend;

/// Re-exports
pub mod prelude {
  pub use crate::backend::{
    disk::{FileBackend, FileBackendBuilder},
    s3::{S3Backend, S3BackendBuilder, S3Config, S3ConfigBuilder},
    Backend,
  };
  pub use anyhow::{Context, Result};
}
