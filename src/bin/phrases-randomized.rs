use crate::tutorials::string_search::core::*;
use anyhow::Error;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::hint::black_box;
use tutorials::utils::generate::phrases_randomized;

extern crate tutorials;

pub fn main() -> Result<(), Error> {
    static PHRASE_COUNT: i32 = 20;
    // static INVOCATION_COUNT: i32 = 1_000;

    let rng_phrases = phrases_randomized(PHRASE_COUNT.try_into()?, 3);
    if let Ok(phrases) = rng_phrases {
        // for _loop_idx in 0..INVOCATION_COUNT {
        //     let _ = black_box(());
        // }

        let phrase_text = phrases
            .iter()
            .map(|el| format!("\"{}\", ", el))
            .collect::<String>();
        // dbg!(&phrase_text);
        println!("Phrases generated [{}]: {}", &phrases.len(), &phrase_text);
    }

    Ok(())
}
