use egui::{Color32, Pos2};

use munote::duration::Duration;

#[derive(Debug)]
pub struct Symbol {
    pub pos: Pos2,
    pub glyph: String,
    pub color: Color32,
    pub duration: Duration,
    pub rotated: bool,
}

impl Symbol {
    pub fn new(pos: Pos2, glyph: impl ToString, color: Color32, duration: Duration) -> Self {
        Self {
            pos,
            glyph: glyph.to_string(),
            color,
            duration,
            rotated: false
        }
    }

    pub fn rotate(mut self) -> Self {
        self.rotated = true;
        self
    }
}