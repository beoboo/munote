use cucumber::*;
use munote::score::Score;
use std::collections::HashMap;

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
