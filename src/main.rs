use reqwest::blocking::get;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;

const TOKENS: [&str; 3] = [
    "meta-token.near",
    "token.v2.ref-finance.near",
    "token.burrow.near",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = get("https://indexer.ref.finance/list-token-price")?.text()?;
    let prices: Value = serde_json::from_str(&response)?;
    let mut picked_tokens = HashMap::new();

    for &token in &TOKENS {
        if let Some(value) = prices.get(token) {
            picked_tokens.insert(token, value.clone());
        }
    }

    let file = File::create("./ref-prices.json")?;
    serde_json::to_writer(file, &picked_tokens)?;

    Ok(())
}
