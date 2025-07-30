#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Error;
use hyper::header::HeaderValue;
use std::fmt;
use std::ops;
use tower_http_0_2_4::cors::{CorsLayer, Origin};

struct CorsOrigins<'a>(pub(crate) &'a Vec<String>);

impl IntoIterator for CorsOrigins<'_> {
    type Item = HeaderValue;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut collection: Vec<HeaderValue> = vec![];
        let _result: Vec<String> = self
            .0
            .iter()
            .map(|x| {
                collection.push(HeaderValue::from_str(x).unwrap());
                x.to_string()
            })
            .collect();

        collection.into_iter()
    }
}

impl<'a> fmt::Display for CorsOrigins<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter().fold(Ok(()), |result, origin| {
            result.and_then(|_| writeln!(f, "{}", origin))
        })
    }
}

impl ops::Deref for CorsOrigins<'_> {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[tokio::main]
async fn main() {
    let production_origins = vec!["https://example.com".parse().unwrap()];

    tracing::info!(
        "[ OK ]: CORS access enabled! {}",
        CorsOrigins(&production_origins)
    );

    // In this example we create a Newtype to implement `IntoIterator`.  This is because Axum's `Origin::list()` function takes a generic parameter with a trait bound of `IntoIterator<Item = HeaderValue>`.
    // We therefore implement the `IntoIterator` trait on our Newtype so that it can be passed as an argument to the `Origin::list()` function.
    CorsLayer::new().allow_origin(Origin::list(CorsOrigins(&production_origins)));
}
