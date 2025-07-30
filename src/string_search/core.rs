//! Challenge:
//!
//! - Assume two users are using a site like eBay, but the concept is that user Bob can post an item for sale via a text input field and offer a sell price.
//! - The bidding action is performed purely in memory, so ignore I/O and other complexities.
//! - Think in terms of C++ and pointers.  This is a quiz on simple data structures.
//! - Optimise for Big O performance.
//!
//! Design notes:
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
//! Benchmarking and Profiling:
//!
//! Refer to `benchmarking/README.md`
//!
//! Tests:
//!
//! Run `test-string-search.sh`
//!
use anyhow::anyhow;
use anyhow::Error;
use std::collections::HashMap;

pub struct Entries(pub HashMap<String, f64>);

impl Entries {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn add(&mut self, info: (String, f64)) {
        let (text, amount) = info;
        self.0.insert(text, amount);
    }

    pub fn add_many<'a>(&mut self, items: impl IntoIterator<Item = (&'a str, f64)>) {
        let new: HashMap<String, f64> =
            items.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
        self.0 = new;
    }

    pub fn search(&mut self, needle: &str, offered_amount: f64) -> Result<f64, Error> {
        log::debug!("Number of items to search: {}", self.0.len());

        if let Some(cost) = self.0.get(needle) {
            if offered_amount >= *cost {
                Ok(*cost)
            } else {
                return Err(anyhow!(
                    "Item {} costs more than your offer of ${}",
                    &needle,
                    &offered_amount
                ));
            }
        } else {
            return Err(anyhow!("Item {} is not available!", &needle));
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::Entries;
    use crate::utils::generate::phrases;
    use rand::{thread_rng, Rng};
    use std::collections::HashMap;
    use std::hint::black_box;

    pub fn rng_amount() -> f64 {
        let mut rng = thread_rng();
        rng.gen_range(19.0..1.0e2)
    }

    #[test]
    fn match_existing_available_item() {
        static PHRASE_COUNT: i32 = 10_000;
        let rng_phrases = phrases(PHRASE_COUNT);

        if let Ok(phrases) = rng_phrases {
            let items: HashMap<&str, f64> = phrases
                .iter()
                .map(|phrase| (phrase.as_str(), rng_amount()))
                .collect();
            let hm_keys = &items.keys().map(|el| el.to_owned()).collect::<Vec<&str>>();

            let mut entries = Entries::new();
            entries.add_many(black_box(items));
            assert!(entries.len() > 0);

            // Success, the buyer gets an instant match!
            let _ = black_box(entries.search(hm_keys[0], 120.00).unwrap());
        }
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
