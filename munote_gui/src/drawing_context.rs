use eframe::epaint::Shape;
use egui::{Align2, Color32, FontId, Painter, Pos2, pos2, Stroke};
use tracing::info;

use munote::chord::Chord;
use munote::duration::Duration;
use munote::note::{Note, StemDirection};
use munote::rest::Rest;
use munote::symbols::Symbols;
use munote::tag::Tag;
use munote::tag_id::TagId;
use munote::visitor::Visitor;
use munote::voice::Voice;
use crate::ui::rotated_text::RotatedText;
use crate::symbol::Symbol;

pub struct DrawingContext {
    pub painter: Painter,
    pub origin: Pos2,
    pub position: Pos2,
    pub width: f32,
    pub height: f32,
    pub color: Color32,
    pub font_size: f32,
    pub font_id: FontId,
}

impl DrawingContext {
    pub fn new(
        painter: Painter,
        origin: Pos2,
        width: f32,
        height: f32,
        color: Color32,
        font_size: f32,
        font_id: FontId,
    ) -> Self {
        Self {
            painter,
            origin,
            position: origin,
            width,
            height,
            color,
            font_size,
            font_id,
        }
    }
}

impl Visitor for DrawingContext {
    fn on_note(&mut self, note: &Note) {
        let size = self.font_size;
        let head_height = size / 4.0;

        // Adjust to A1
        let mut y = -head_height;

        // Adjust to actual pitch
        y -= (note.diatonic_pitch() as f32) * head_height / 2.0;

        let mut symbol = Symbol::new(pos2(size, y),
            Symbols::note_from_duration(note.duration),
            self.color,
            note.duration,
        );

        if note.stem_direction() == StemDirection::Down {
            symbol = symbol.rotate();
            // let symbol = Symbol::new(pos2(size, y),
            //     Symbols::note_head_from_duration(note.duration),
            //     self.color,
            //     note.duration,
            // );
            //
            // self.render_symbol(symbol);
        }

        self.render_symbol(symbol);


        // // Stem down
        // if note.has_stem() && note.stem_direction() == StemDirection::Down {
        //     // glyph.push();
        //     let symbol = Symbol::new(
        //         pos2(size, y + size - head_height / 2.0),
        //         Symbols::get("COMBINING STEM"),
        //         self.color,
        //         note.duration,
        //     );
        //
        //     self.render_symbol(symbol);
        //
        //     if note.num_beams() > 0 {
        //         let symbol = Symbol::new(
        //             pos2(size - 4.0, y),
        //             Symbols::beams_from_duration(note.duration),
        //             self.color,
        //             note.duration, // FIXME
        //         ).rotate();
        //
        //         self.render_symbol(symbol);
        //     }
        // }
        //
        // let symbol = Symbol::new(pos2(size, y),
        //     Symbols::note_head_from_duration(note.duration),
        //     self.color,
        //     note.duration,
        // );
        //
        // self.render_symbol(symbol);
        //
        // // Stem up
        // if note.has_stem() && note.stem_direction() == StemDirection::Up {
        //     // glyph.push();
        //     let symbol = Symbol::new(
        //         pos2(size - 8.0, y),
        //         Symbols::get("COMBINING STEM"),
        //         self.color,
        //         note.duration);
        //
        //     self.render_symbol(symbol);
        //
        //     if note.num_beams() > 0 {
        //         let symbol = Symbol::new(
        //             pos2(size - 4.0, y),
        //             Symbols::beams_from_duration(note.duration),
        //             self.color,
        //             note.duration, // FIXME
        //         );
        //
        //         self.render_symbol(symbol);
        //
        //         let pos = &mut self.position;
        //         pos.x += size / 4.0;
        //     }
        // }

        for _ in 0..usize::from(note.dots) {
            let symbol = Symbol::new(
                pos2(size, y + head_height / 2.0),
                Symbols::get("COMBINING AUGMENTATION DOT"),
                self.color,
                note.duration, // FIXME
            );

            self.render_symbol(symbol);

            let pos = &mut self.position;
            pos.x += size / 8.0;
        }
    }

    fn on_chord(&mut self, _chord: &Chord) {
        todo!()
    }

    fn on_rest(&mut self, rest: &Rest) {
        let size = self.font_size;
        let head_height = size / 4.0;
        let mut glyph = String::from(Symbols::rest_from_duration(rest.duration));

        // Adjust to A1
        let mut y = -head_height;
        // let y = 0.0;

        let symbol = Symbol::new(
            pos2(size, y),
            glyph,
            self.color,
            rest.duration,
        );

        self.render_symbol(symbol);

        // for _ in 0..usize::from(rest.dots) {
        //     let symbol = Symbol {
        //         pos: pos2(size, y + head_height / 2.0),
        //         glyph: Symbols::get("COMBINING AUGMENTATION DOT"),
        //         color: self.color,
        //         duration: rest.duration, // FIXME
        //     };
        //
        //     self.render_symbol(symbol);
        //
        //     let pos = &mut self.position;
        //     pos.x += size / 8.0;
        // }
    }

    fn on_tag(&mut self, tag: &Tag) {
        match tag.id {
            TagId::Clef => self.render_clef(tag),
            id => unimplemented!("{id:?}")
        }
    }

    fn on_voice(&mut self, _voice: &Voice) {
        // let symbol = Symbol::new(
        //     pos2(0.0, 0.0),
        //     Symbols::get("FIVE-LINE STAFF"),
        //     self.color,
        //     Duration::default(),
        // );
        //
        // self.render_symbol(symbol);


        let mut lines = vec![];
        let pos = self.position;
        let width = self.width;

        for i in 0..5 {
            let y = pos.y + i as f32 * self.font_size / 4.0;

            lines.push(Shape::line_segment(
                [Pos2::new(pos.x, y), Pos2::new(pos.x + width, y)],
                Stroke::new(1.5, self.color),
            ));
        }

        self.painter.extend(lines);
        self.position.y += self.font_size / 2.0 - 2.0;
    }

    fn on_staff_begin(&mut self) {}

    fn on_staff_end(&mut self) {
        let origin = self.origin;
        let pos = self.position;
        let width = self.width;

        // Bar ending
        self.painter.text(
            pos2(origin.x + width - 8.0, pos.y),
            Align2::LEFT_CENTER,
            Symbols::get("FINAL BARLINE"),
            self.font_id.clone(),
            self.color,
        );
    }
}

impl DrawingContext {
    fn render_clef(&mut self, _tag: &Tag) {
        let symbol = Symbol::new(
            pos2(0.0, 0.0),
            Symbols::get("G CLEF"),
            self.color,
            Duration::default(),
        );

        self.render_symbol(symbol);
    }

    fn render_symbol(&mut self, symbol: Symbol) {
        let pos = &mut self.position;
        let symbol_pos = symbol.pos;

        let rect = if symbol.rotated {
            self.painter.rotated_text(
                pos2(pos.x + symbol_pos.x, pos.y + symbol_pos.y),
                Align2::LEFT_CENTER,
                format!("\u{2794}{}", symbol.glyph),
                self.font_id.clone(),
                symbol.color,
                180.0,
            )
        } else {
            self.painter.text(
                pos2(pos.x + symbol_pos.x, pos.y + symbol_pos.y),
                Align2::LEFT_CENTER,
                symbol.glyph.clone(),
                self.font_id.clone(),
                symbol.color,
            )
        };

        info!("Symbol: \u{2794}{}, rect: {:?}", &symbol.glyph, rect);

        // if self.draw_bounds {
        // self.painter.rect(rect, 0.0, Color32::TRANSPARENT, Stroke::new(1.0, Color32::RED));
        // }

        pos.x += rect.width();
    }
}