#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Error;
use async_trait::async_trait;
use std::fmt::Debug;
use std::io::Write;
use tokio::io::AsyncWriteExt;

#[async_trait]
pub trait PersistantBackend: Debug {
    async fn insert(&mut self, data: Vec<u8>) -> Result<(), Error>;
}

pub mod backend {
    use super::*;
    use tokio::fs::File;

    #[derive(Debug)]
    pub struct FileStore {}

    #[async_trait]
    impl PersistantBackend for FileStore {
        async fn insert(&mut self, data: Vec<u8>) -> Result<(), Error> {
            let mut file = File::create("data.txt").await?;
            let text = data
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<String>>()
                .join(",");

            file.write_all(text.as_bytes()).await?;
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct PersistableBackend {
    backend: Box<dyn PersistantBackend>,
}

#[tokio::main]
async fn main() {
    todo!()
}

pub async fn insert_data(backend: &mut PersistableBackend, data: Vec<u8>) -> Result<(), Error> {
    backend.backend.insert(data).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::AsBytes;
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;

    #[tokio::test]
    async fn test_insert_data() {
        let file_store = backend::FileStore {};
        let mut backend = PersistableBackend {
            backend: Box::new(file_store),
        };

        let data = vec![1, 2, 3];
        insert_data(&mut backend, data.clone())
            .await
            .expect("Failed to insert data");

        let mut file = File::open("data.txt").await.expect("Failed to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .await
            .expect("Failed to read file");

        let text = data
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<String>>()
            .join(",");

        assert_eq!(text.as_bytes(), buffer);

        // Cleanup
        tokio::fs::remove_file("data.txt")
            .await
            .expect("Failed to remove file");
    }
}
