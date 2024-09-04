use pluggable_storage::prelude::*;

extern crate pluggable_storage;

#[tokio::main]
async fn main() -> Result<()> {
  // This will be replaced with a source of data using Serde
  let data = String::default();

  let config = S3ConfigBuilder::default()
    // block
    .bucket("pluggable_storage_demo")
    .file_name("output.json")
    .build()?;
  let backend_builder: S3BackendBuilder<S3Backend> = S3BackendBuilder::new(config);
  let backend = &mut backend_builder.build();

  backend.persist(data.as_bytes()).await?;

  Ok(())
}
