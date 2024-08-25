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
use rand::distributions::{Alphanumeric, DistString};
use rand::prelude::SliceRandom;
use rand::{thread_rng};

pub fn phrases(count: i32) -> Vec<String> {
    let mut rng = thread_rng();
    let mut nums: Vec<i32> = (1..8).collect();

    let data: Vec<String> = (0..count)
        .into_iter()
        .map(|_| {
            nums.shuffle(&mut rng);

            let phrase: Vec<String> = (0..nums[0])
                .into_iter()
                .map(|_| -> String {
                    nums.shuffle(&mut rng);
                    Alphanumeric.sample_string(&mut rand::thread_rng(), nums[0] as usize)
                })
                .collect();

            phrase.join(" ")
        })
        .collect();

    data
}
