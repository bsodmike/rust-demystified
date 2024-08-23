// #![allow(unused_imports, dead_code, unused_variables)]

use anyhow::anyhow;
use anyhow::Error;
use std::{collections::HashMap, sync::LazyLock};
use tokio::sync::Mutex;

pub static ENTRY_MAP: LazyLock<Mutex<Option<Entries>>> =
    LazyLock::new(|| Mutex::new(Some(Entries(HashMap::new()))));

pub struct Entries(pub HashMap<String, f64>);

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

pub mod benchmarking {
    pub fn contains(haystack: &[String], needle: String) -> bool {
        haystack.iter().any(|x| x == &needle)
    }
}
