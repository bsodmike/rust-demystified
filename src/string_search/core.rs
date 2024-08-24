use anyhow::anyhow;
use anyhow::Error;
use rand::seq::IteratorRandom;
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
        let mut existing_data = self.0.clone();
        existing_data.insert(text, amount);

        self.0 = existing_data;
    }

    pub fn add_many<'a>(&mut self, items: impl IntoIterator<Item = (&'a str, f64)>) {
        let new: HashMap<String, f64> =
            items.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
        self.0 = new;
    }

    pub fn search(&mut self, input: &str, amount: f64) -> Result<(String, f64, f64), Error> {
        let text = input.to_string();
        let needle = text;
        let haystack: Vec<String> = self.0.iter().map(|el| el.0.to_string()).collect();
        log::debug!("Number of items to search: {}", self.0.len());

        if haystack.contains(&needle) {
            if let Some(cost) = self.0.get(&needle) {
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
    }
}

pub mod benchmarking {
    pub fn contains(haystack: &[String], needle: String) -> bool {
        haystack.iter().any(|x| x == &needle)
    }
}
