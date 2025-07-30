#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Error;
use pin_project;

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

#[pin_project::pin_project]
pub struct TimedWrapper<Fut: Future> {
    start: Option<Instant>,
    #[pin]
    future: Fut,
}

impl<Fut: Future> TimedWrapper<Fut> {
    pub fn new(future: Fut) -> Self {
        Self {
            future,
            start: None,
        }
    }
}

impl<Fut: Future> Future for TimedWrapper<Fut> {
    type Output = (Fut::Output, Duration);

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut this = self.project();
        // Call the inner poll, measuring how long it took.
        let start = this.start.get_or_insert_with(Instant::now);
        let inner_poll = this.future.as_mut().poll(cx);
        let elapsed = start.elapsed();

        match inner_poll {
            // The inner future needs more time, so this future needs more time too
            Poll::Pending => Poll::Pending,
            // Success!
            Poll::Ready(output) => Poll::Ready((output, elapsed)),
        }
    }
}

// Article: https://blog.cloudflare.com/pin-and-unpin-in-rust/
// Github: https://github.com/adamchalmers/nested-future-example/blob/master/src/main.rs
#[tokio::main]
async fn main() {
    let (resp, time) = TimedWrapper::new(fake_reqwest()).await;
    println!(
        "Got a HTTP {} in {}ms",
        resp.unwrap().status(),
        time.as_millis()
    )
}

// Stubbing out use of `reqwest` with a dummy, just to get this to compile.
struct FakeRequest {}

impl FakeRequest {
    fn status(&self) -> String {
        String::from("")
    }
}
async fn fake_reqwest() -> Result<FakeRequest, Error> {
    let f = FakeRequest {};

    Ok(f)
}
