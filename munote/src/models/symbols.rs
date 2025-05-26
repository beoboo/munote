use std::collections::HashMap;

use anyhow::Result;
use lazy_static::lazy_static;

use crate::duration::Duration;

pub struct Symbols;

impl Symbols {
    pub fn get(key: &str) -> char {
        lazy_static! {
            static ref SYMBOLS: HashMap<&'static str, char> = load_defs(include_str!("../../assets/symbols.yaml"))
                .expect("Could not load symbols");
        }

        *SYMBOLS.get(&key).expect(&format!("Invalid symbol \"{key}\""))
    }

    pub fn note_from_duration(duration: Duration) -> char {
        match (duration.num, duration.denom) {
            (1, 1) => Self::get("WHOLE NOTE"),
            (1, 2) => Self::get("HALF NOTE"),
            (1, 4) => Self::get("QUARTER NOTE"),
            (1, 8) => Self::get("EIGHTH NOTE"),
            (1, 16) => Self::get("SIXTEENTH NOTE"),
            (1, 32) => Self::get("THIRTY-SECOND NOTE"),
            (1, 64) => Self::get("SIXTY-FOURTH NOTE"),
            (1, 128) => Self::get("ONE HUNDRED TWENTY-EIGHTH NOTE"),
            _ => unimplemented!("Invalid note duration: {:?}", duration),
        }
    }

    pub fn note_head_from_duration(duration: Duration) -> char {
        match (duration.num, duration.denom) {
            (1, 1) => Self::get("WHOLE NOTE"),
            (1, 2) => Self::get("VOID NOTEHEAD"),
            _ => Self::get("NOTEHEAD BLACK"),
        }
    }

    pub fn beams_from_duration(duration: Duration) -> char {
        match (duration.num, duration.denom) {
            (1, 8) => Self::get("COMBINING FLAG-1"),
            (1, 16) => Self::get("COMBINING FLAG-2"),
            (1, 32) => Self::get("COMBINING FLAG-3"),
            (1, 64) => Self::get("COMBINING FLAG-4"),
            (1, 128) => Self::get("COMBINING FLAG-5"),
            _ => unimplemented!("Invalid note duration: {:?}", duration),
        }
    }

    pub fn rest_from_duration(duration: Duration) -> char {
        match (duration.num, duration.denom) {
            (1, 1) => Self::get("WHOLE REST"),
            (1, 2) => Self::get("HALF REST"),
            (1, 4) => Self::get("QUARTER REST"),
            (1, 8) => Self::get("EIGHTH REST"),
            (1, 16) => Self::get("SIXTEENTH REST"),
            (1, 32) => Self::get("THIRTY-SECOND REST"),
            (1, 64) => Self::get("SIXTY-FOURTH REST"),
            (1, 128) => Self::get("ONE HUNDRED TWENTY-EIGHTH REST"),
            _ => unimplemented!("Invalid note duration: {:?}", duration),
        }
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
