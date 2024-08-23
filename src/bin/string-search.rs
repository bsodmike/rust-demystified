//! Challenge:
//!
//! - Assume two users are using a site like eBay, but the concept is that user Bob can post an item for sale via a text input field and offer a sell price.
//! - The bidding action is performed purely in memory, so ignore I/O and other complexities.
//! - Think in terms of C++ and pointers.  This is a quiz on simple data structures.
//! - Optimise for Big O performance.
//!
//! Design notes
//!
//! - User input is assumed to be all lower case, alphanumerals only, with single spaces.  The assumption is that this binary operates on clean data.  Validating and sanitising input is a further refinement.
//! - TBD
//!
//! Optimisations:
//!
//! Create small and big benchmarks:
//!
//! - Use https://github.com/bheisler/criterion.rs
//! - Use https://github.com/nvzqz/divan
//!
//! Running Tests
//!
//!
//! Benchmarking and Profiling
//!
//! - Run with `cargo flamegraph --root --unit-test string-search -- tests::match_existing_available_item`.
//!
#![allow(unused_imports, dead_code, unused_variables)]

use crate::benchmarking::contains;
use anyhow::anyhow;
use anyhow::Error;
use std::cell::RefCell;
use std::hash::Hash;
use std::hint::black_box;
use std::{
    collections::HashMap,
    io::{BufRead, Write},
    ops::Deref,
    pin::Pin,
    sync::LazyLock,
};
use tokio::sync::Mutex;

pub static ENTRY_MAP: LazyLock<Mutex<Option<Entries>>> =
    LazyLock::new(|| Mutex::new(Some(Entries(HashMap::new()))));

pub struct Entries(HashMap<String, f64>);

impl Entries {
    pub async fn add(text: String, amount: f64) {
        let index_lock: &mut Option<Entries> = &mut *ENTRY_MAP.lock().await;
        if let Some(entries) = index_lock {
            let mut existing_data = entries.0.clone();
            existing_data.insert(text, amount);

            entries.0 = existing_data;
        }
    }

    pub async fn add_many(items: HashMap<&str, f64>) {
        let index_lock: &mut Option<Entries> = &mut *ENTRY_MAP.lock().await;
        if let Some(entries) = index_lock {
            let new: HashMap<String, f64> = items
                .into_iter()
                .map(|el| (el.0.to_string(), el.1))
                .collect();
            entries.0 = new;
        }
    }

    pub async fn search(
        input: &str,
        amount: f64,
        benchmark: bool,
    ) -> Result<(String, f64, f64), Error> {
        let text = input.to_string();

        let index_lock: &mut Option<Entries> = &mut *ENTRY_MAP.lock().await;
        if let Some(entries) = index_lock {
            // println!("Number of items to search: {}", entries.0.len());

            let needle = text;
            let haystack: Vec<String> = entries
                .0
                .clone()
                .into_iter()
                .map(|el| el.0.to_string())
                .collect();

            if haystack.contains(&needle) {
                if let Some(cost) = entries.0.get(&needle) {
                    if amount >= *cost {
                        Ok((needle, amount, *cost))
                    } else {
                        return Err(anyhow!(
                            "Item {} costs more than your offer of ${}",
                            &needle,
                            &amount
                        ));
                    }
                } else {
                    unreachable!()
                }
            } else {
                return Err(anyhow!("Item {} is not available!", &needle));
            }
        } else {
            // This would be classed as an internal error, so ideally I would mark this as `unreachable!()`.
            return Err(anyhow!("Internal error"));
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    Ok(())
}

pub mod benchmarking {
    pub fn contains(haystack: &[String], needle: String) -> bool {
        haystack.iter().any(|x| x == &needle)
    }
}

#[cfg(test)]
pub mod tests {
    extern crate tutorials;

    use super::*;
    use tutorials::utils::generate::{float_nums, phrases};

    #[tokio::test]
    async fn match_existing_available_item() {
        static PHRASE_COUNT: i32 = 10_000_000;
        let rng_phrases = phrases(PHRASE_COUNT);
        let items: HashMap<&str, f64> = rng_phrases
            .iter()
            .map(|phrase| (phrase.as_str(), float_nums()))
            .collect();

        // let items: HashMap<&str, f64> =
        //     HashMap::from([("red apple", 20.0), ("ferrari", 32.1), ("banana", 12.99)]);
        // dbg!(&items);
        let hm_keys = &items.clone().into_keys().collect::<Vec<&str>>();
        Entries::add_many(items).await;

        {
            let index_lock: &mut Option<Entries> = &mut *ENTRY_MAP.lock().await;
            if let Some(entries) = index_lock {
                dbg!(&entries.0);
                assert!(entries.0.len() > 0);
            };
        }

        // Success, the buyer gets an instant match!

        let (item, bid, ask) = black_box(Entries::search(hm_keys[0], 120.00, true).await.unwrap());
    }

    #[tokio::test]
    async fn panic_expect_error_for_low_bid() {
        let items = HashMap::from([("red apple", 20.0), ("ferrari", 32.1), ("banana", 12.99)]);
        Entries::add_many(items).await;

        {
            let index_lock: &mut Option<Entries> = &mut *ENTRY_MAP.lock().await;
            if let Some(entries) = index_lock {
                dbg!(&entries.0);
                assert!(entries.0.len() > 0);
            };
        }

        if let Err(err) = Entries::search("banana", 8.23, false).await {
            assert_eq!(
                err.to_string(),
                String::from("Item banana costs more than your offer of $8.23")
            )
        }
    }

    #[should_panic]
    #[tokio::test]
    async fn handle_mismatching_category() {
        let items = HashMap::from([("red apple", 20.0), ("ferrari", 32.1), ("banana", 12.99)]);
        Entries::add_many(items).await;

        {
            let index_lock: &mut Option<Entries> = &mut *ENTRY_MAP.lock().await;
            if let Some(entries) = index_lock {
                dbg!(&entries.0);
                assert!(entries.0.len() > 0);
            };
        }

        Entries::search("fruit", 20.00, false).await.unwrap();
    }

    #[tokio::test]
    async fn panic_mismatching_category_partial_text() {
        let items = HashMap::from([("red apple", 20.0), ("ferrari", 32.1), ("banana", 12.99)]);
        Entries::add_many(items).await;

        {
            let index_lock: &mut Option<Entries> = &mut *ENTRY_MAP.lock().await;
            if let Some(entries) = index_lock {
                dbg!(&entries.0);
                assert!(entries.0.len() > 0);
            };
        }

        if let Err(err) = Entries::search("red appl", 8.23, false).await {
            assert_eq!(
                err.to_string(),
                String::from("Item red appl is not available!")
            )
        }
    }
}
