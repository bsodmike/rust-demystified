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

    pub fn add(&mut self, text: String, amount: f64) {
        let mut existing_data = self.0.clone();
        existing_data.insert(text, amount);

        self.0 = existing_data;
    }

    pub fn add_many(&mut self, items: HashMap<&str, f64>) {
        let new: HashMap<String, f64> = items
            .into_iter()
            .map(|el| (el.0.to_string(), el.1))
            .collect();
        self.0 = new;
    }

    pub fn search(&mut self, input: &str, amount: f64) -> Result<(String, f64, f64), Error> {
        let text = input.to_string();

        // println!("Number of items to search: {}", entries.0.len());

        let needle = text;
        let haystack: Vec<String> = self
            .0
            .clone()
            .into_iter()
            .map(|el| el.0.to_string())
            .collect();

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
