#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Error;
use std::pin::Pin;

async fn blocks(item: Item) -> Result<(), Error> {
    let my_string = "foo".to_string();

    let future_one = async {
        // ...
        println!("{my_string}");
    };

    let future_two = async {
        // ...
        println!("{my_string}");
    };

    // Run both futures to completion, printing "foo" twice:
    let ((), ()) = futures::join!(future_one, future_two);

    Ok(())
}

struct Item {}

#[tokio::main]
async fn main() {
    let item = Item {};
    // let pinned = Box::pin(item);

    let _ = blocks(item).await;
}
