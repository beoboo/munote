use egui::{Color32, pos2, Pos2};
use munote::display_event::DisplayEvent;
use munote::duration::Duration;
use munote::event::Event;
use munote::note::Note;

use munote::symbols::Symbols;
use munote::tag::Tag;

use crate::context::Context;

#[derive(Debug)]
pub struct Symbol {
    pub pos: Pos2,
    pub glyph: char,
    pub color: Color32,
    pub duration: Duration,
}

impl Symbol {
    pub fn from_clef(tag: &Tag, context: &Context) -> Self {
        Self {
            pos: pos2(context.font_size, 0.0),
            glyph: Symbols::get("G CLEF").unwrap(),
            color: context.color,
            duration: Duration::default(),
        }
    }

    pub fn from_event(event: &Box<dyn Event>, context: &Context) -> Option<Self> {
        let size = context.font_size;
        let head_height = size / 4.0;

        if let Some(note) = event.as_any().downcast_ref::<Note>() {
            println!("Displaying {note:?} {}", note.diatonic_pitch());
            // let (x, y) = displayable.adjust();

            // Adjust glyph

            let mut y = -0.0;

            // Adjust to A1
            y -= head_height;

            // Adjust to actual pitch
            y -= (note.diatonic_pitch() as f32) * head_height / 2.0;

            let symbol = Self {
                // pos: pos2(context.font_size, context.font_size - 4.0),
                pos: pos2(size, y),
                glyph: Symbols::get("NOTEHEAD BLACK").unwrap(),
                color: context.color,
                duration: note.duration,
            };

            Some(symbol)
        } else {
            None
        }
    }
}