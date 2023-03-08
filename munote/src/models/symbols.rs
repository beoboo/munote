use anyhow::{anyhow, Result};
use std::collections::HashMap;
use lazy_static::lazy_static;

pub struct Symbols;

impl Symbols {
    pub fn get(key: &str) -> Result<char> {
        lazy_static! {
            static ref SYMBOLS: HashMap<&'static str, char> = load_defs(include_str!("../../assets/symbols.yaml"))
                .expect("Could not load symbols");
        }

        SYMBOLS.get(&key).ok_or_else(|| anyhow!("Invalid symbol \"{key}\"")).and_then(|c| Ok(*c))
    }
}

fn load_defs(input: &str) -> Result<HashMap<&str, char>> {
    let parsed = serde_yaml::from_str::<HashMap<&str, &str>>(input)?;

    let defs = parsed.into_iter()
        .map(|(c, v)| (u32::from_str_radix(c, 16).unwrap(), v))
        .map(|(c, v)| (v, char::from_u32(c).unwrap()))
        .collect::<HashMap<_, _>>();

    Ok(defs)
}
