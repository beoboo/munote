use anyhow::Result;

use crate::{
    accidentals::Accidentals,
    dots::Dots,
    duration::Duration,
    note::{Diatonic, Note},
};

pub struct NoteParser;

impl NoteParser {
    pub fn parse_note(&self, _input: impl Into<String>) -> Result<Note> {
        Ok(Note::new(
            Diatonic::A,
            Accidentals::Natural,
            1,
            Duration::default(),
            Dots::None,
        ))
    }
}

#[cfg(test)]
mod tests {}
