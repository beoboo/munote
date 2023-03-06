use std::collections::HashMap;
use cucumber::*;
use munote::score::Score;

#[derive(Debug, Default, World)]
pub struct MusicWorld {
    pub files: HashMap<String, String>,
    pub score: Option<Score>,
}

impl MusicWorld {
    pub fn count_staffs(&self) -> usize {
        if let Some(score) = &self.score {
            score.staffs.len()
        } else {
            0
        }
    }
}
