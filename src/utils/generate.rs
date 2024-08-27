//! The MIT License (MIT)
//!
//! Copyright (c) Michael de Silva (https://desilva.io/about)
//! Email: michael@cyberdynea.io
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy of
//! this software and associated documentation files (the "Software"), to deal in
//! the Software without restriction, including without limitation the rights to
//! use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
//! the Software, and to permit persons to whom the Software is furnished to do so,
//! subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
//! FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
//! COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
//! IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
//! CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
#[allow(unused_imports)]
use anyhow::{Context, Error};
use rand::distributions::{Alphanumeric, DistString};
use rand::{thread_rng, Rng};
use tokio::time::error::Elapsed;

pub fn phrases(count: i32) -> Result<Vec<String>, Error> {
    let rng_word_length = 10;
    let rng_word_length_min = 3;

    (0..count)
        .map(|_| thread_rng().gen_range(rng_word_length_min..rng_word_length) as usize)
        .map(|length| Alphanumeric.sample_string(&mut thread_rng(), length))
        // NOTE: Optionally, while this example does not generate any empty words, if we wanted to filter through such a scenario, it can be done using a filter to generate an `Option`.
        // .map(|word| Some(word).filter(|word| word.len() != 0))
        // .map(|word| word.context(anyhow::anyhow!("Word length is 0!")))
        .map(|word| Ok(word))
        .collect()
}

pub fn phrases_randomized(count: usize, words_per_phrase: usize) -> Result<Vec<String>, Error> {
    let rng_word_length = 10;
    let rng_word_length_min = 3;

    Ok((0..count)
        .map(|_| {
            (0..words_per_phrase)
                .map(|_| thread_rng().gen_range(rng_word_length_min..rng_word_length) as usize)
                .map(|length| Alphanumeric.sample_string(&mut thread_rng(), length))
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect())
}
