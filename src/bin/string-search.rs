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
//! Running Tests, Benchmarking and Profiling
//!
//! - Tests: `cargo t --bin string-search`
//! - Flamegraph: `cargo flamegraph --root --flamechart --verbose --bin string-search-heaptrack`
//! - Heaptrack: TBD
//!
// #![allow(unused_imports, dead_code, unused_variables)]

use anyhow::Error;

extern crate tutorials;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use crate::tutorials::string_search::core::*;
    use crate::tutorials::utils::generate::{float_nums, phrases};
    use std::collections::HashMap;
    use std::hint::black_box;

    #[test]
    fn match_existing_available_item() {
        static PHRASE_COUNT: i32 = 10_000;
        let rng_phrases = phrases(PHRASE_COUNT);
        let items: HashMap<&str, f64> = rng_phrases
            .iter()
            .map(|phrase| (phrase.as_str(), float_nums()))
            .collect();
        let hm_keys = &items.clone().into_keys().collect::<Vec<&str>>();

        let mut entries = Entries::new();
        entries.add_many(black_box(items));
        assert!(entries.len() > 0);

        // Success, the buyer gets an instant match!
        let _ = black_box(entries.search(hm_keys[0], 120.00).unwrap());
    }

    #[test]
    fn panic_expect_error_for_low_bid() {
        let items = HashMap::from([("red apple", 20.0), ("ferrari", 32.1), ("banana", 12.99)]);
        let mut entries = Entries::new();
        entries.add_many(black_box(items));

        if let Err(err) = entries.search("banana", 8.23) {
            assert_eq!(
                err.to_string(),
                String::from("Item banana costs more than your offer of $8.23")
            )
        }
    }

    #[should_panic]
    #[test]
    fn handle_mismatching_category() {
        let items = HashMap::from([("red apple", 20.0), ("ferrari", 32.1), ("banana", 12.99)]);
        let mut entries = Entries::new();
        entries.add_many(black_box(items));

        entries.search("fruit", 20.00).unwrap();
    }

    #[test]
    fn panic_mismatching_category_partial_text() {
        let items = HashMap::from([("red apple", 20.0), ("ferrari", 32.1), ("banana", 12.99)]);
        let mut entries = Entries::new();
        entries.add_many(black_box(items));

        if let Err(err) = entries.search("red appl", 8.23) {
            assert_eq!(
                err.to_string(),
                String::from("Item red appl is not available!")
            )
        }
    }
}
