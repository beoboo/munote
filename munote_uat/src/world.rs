use cucumber::*;
use munote::Score;

#[derive(Debug, Default, World)]
pub struct MusicWorld {
    score: Option<Score>,
}

impl MusicWorld {
    pub fn count_staffs(&self) -> usize {
        0
    }
}
