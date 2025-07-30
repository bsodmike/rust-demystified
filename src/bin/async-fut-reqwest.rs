use async_trait::async_trait;
use backon::{ExponentialBuilder, Retryable};
use bytes::Bytes;
use helpers::with_retry;
use reqwest::{Error, Response};
use retry_strategy::RetryStrategy;
use std::future::Future;

// Newtype wrapper for reqwest::Client (async)
#[derive(Default)]
pub struct ReqwestClient(reqwest::Client);

impl ReqwestClient {
    pub fn new() -> Self {
        Self(reqwest::Client::new())
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.0
    }

    pub fn client_mut(&mut self) -> &mut reqwest::Client {
        &mut self.0
    }
}

// Trait for the HTTP client
#[async_trait]
pub trait HttpClient {
    async fn post(&self, url: &str, api_key: &str, body: &[u8]) -> Result<Response, Error>;
}

// Implement HttpClient for the newtype with exponential backoff
#[async_trait]
#[allow(clippy::redundant_closure)]
impl HttpClient for ReqwestClient {
    // Alternatively, we could also pass `body: Bytes` and inside the async closure use `body.clone()`, as this is what reqwest expects.
    async fn post(&self, url: &str, api_key: &str, body: &[u8]) -> Result<Response, Error> {
        let retry_strategy = RetryStrategy::default().get_config();

        let operation = async || {
            let body = Bytes::copy_from_slice(body);

            self.client()
                .post(url)
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .body(body)
                .send()
                .await
        };

        // Perform retry operation
        with_retry(operation, retry_strategy).await
    }
}

mod helpers {
    use super::*;

    /// Perform the retry operation with the given closure and retry strategy
    pub async fn with_retry<F, Fut>(
        closure: F,
        retry_strategy: ExponentialBuilder,
    ) -> Result<Response, Error>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<Response, Error>>,
    {
        let resp: Result<Response, Error> = closure.retry(retry_strategy).await;

        // TODO: add error handling

        resp
    }
}

mod retry_strategy {
    use backon::ExponentialBuilder;

    const MAX_DELAY_MILLIS: u64 = 1000;
    pub struct RetryStrategy(ExponentialBuilder);

    impl Default for RetryStrategy {
        /// Default backoff strategy. This uses defaults with jitter.
        fn default() -> Self {
            let retry_strategy = ExponentialBuilder::default()
                .with_factor(2_f32)
                .with_max_times(3)
                .with_max_delay(std::time::Duration::from_millis(MAX_DELAY_MILLIS))
                .with_jitter();

            Self(retry_strategy)
        }
    }

    impl RetryStrategy {
        /// Get the backoff configuration.
        pub fn get_config(self) -> ExponentialBuilder {
            self.0
        }
    }
}

#[tokio::main]
async fn main() {}
