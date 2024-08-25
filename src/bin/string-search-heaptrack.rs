use crate::tutorials::string_search::core::*;
use crate::tutorials::utils::generate::phrases;
use anyhow::Error;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::hint::black_box;

pub fn rng_amount() -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(19.0..1.0e2)
}
extern crate tutorials;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    static PHRASE_COUNT: i32 = 10_000_000;
    let rng_phrases = phrases(PHRASE_COUNT);
    if let Ok(phrases) = rng_phrases {
        let items: HashMap<&str, f64> = phrases
            .iter()
            .map(|phrase| (phrase.as_str(), rng_amount()))
            .collect();

        let hm_keys = &items.clone().into_keys().collect::<Vec<&str>>();

        let mut entries = Entries::new();
        entries.add_many(black_box(items));

        // Success, the buyer gets an instant match!
        let _ = black_box(
            entries
                .search(hm_keys[0], 120.00)
                .expect("Unable to search through entries!"),
        );

        // let phrase_text = phrases
        //     .iter()
        //     .map(|el| format!("\"{}\", ", el))
        //     .collect::<String>();
        println!("Phrases generated: {}", &phrases.len());
    }

    Ok(())
}
