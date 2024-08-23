use crate::tutorials::string_search::core::*;
use crate::tutorials::utils::generate::{float_nums, phrases};
use anyhow::Error;
use std::collections::HashMap;
use std::hint::black_box;

extern crate tutorials;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    static PHRASE_COUNT: i32 = 50;
    let rng_phrases = phrases(PHRASE_COUNT);
    let items: HashMap<&str, f64> = rng_phrases
        .iter()
        .map(|phrase| (phrase.as_str(), float_nums()))
        .collect();

    let hm_keys = &items.clone().into_keys().collect::<Vec<&str>>();
    Entries::add_many(items);

    {
        if let Ok(entries) = &mut ENTRY_MAP.lock() {
            assert!(entries.0.len() > 0);
        };
    }

    // Success, the buyer gets an instant match!
    let _ =
        black_box(Entries::search(hm_keys[0], 120.00).expect("Unable to search through entries!"));
    Ok(())
}
