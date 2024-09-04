use pluggable_storage::prelude::*;

extern crate pluggable_storage;

#[tokio::main]
async fn main() -> Result<()> {
  // This will be replaced with a source of data using Serde
  let data = String::default();
  let backend_builder: FileBackendBuilder<FileBackend> = FileBackendBuilder::new("./output/output.json");
  let backend = &mut backend_builder.build();

  backend.persist(data.as_bytes()).await?;

  Ok(())
}
