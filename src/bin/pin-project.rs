#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Error;
use pin_project::pin_project;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr;

#[derive(Default)]
struct BagOfApples {
    _require_pin: std::marker::PhantomPinned,
}

impl BagOfApples {
    fn sell_one(self: Pin<&mut Self>) {
        println!("Sold an apple!");
    }
}

#[derive(Default)]
struct BagOfOranges {
    _require_pin: std::marker::PhantomPinned,
}

impl BagOfOranges {
    fn sell_one(self: Pin<&mut Self>) {
        println!("Sold an orange!");
    }
}

#[derive(Default)]
struct BagOfBananas {
    _require_pin: std::marker::PhantomPinned,
}

impl BagOfBananas {
    fn sell_one(self: Pin<&mut Self>) {
        println!("Sold a banana!");
    }
}

// ### Projected variant.
#[pin_project]
struct FruitStand {
    #[pin]
    apples: BagOfApples,
    #[pin]
    oranges: BagOfOranges,
    #[pin]
    bananas: BagOfBananas,

    total_sold: usize,
}

impl FruitStand {
    fn sell_one_of_each(mut self: Pin<&mut Self>) {
        let self_projected = self.as_mut().project();

        self_projected.apples.sell_one();
        self_projected.oranges.sell_one();
        self_projected.bananas.sell_one();
        *self_projected.total_sold += 3;
    }

    fn new() -> Pin<Box<Self>> {
        let this = Self {
            apples: BagOfApples {
                _require_pin: std::marker::PhantomPinned,
            },
            oranges: BagOfOranges {
                _require_pin: std::marker::PhantomPinned,
            },
            bananas: BagOfBananas {
                _require_pin: std::marker::PhantomPinned,
            },
            total_sold: 0,
        };

        Box::pin(this)
    }
}

// `pin_project` on struct fields
// Article: https://sander.saares.eu/2024/11/06/why-is-stdpinpin-so-weird/
// Github: https://github.com/sandersaares/pins-in-rust/blob/main/examples/05_project.rs
#[tokio::main]
pub async fn main() {
    let money_jar = &mut FruitStand::new();

    money_jar.as_mut().sell_one_of_each();
}
