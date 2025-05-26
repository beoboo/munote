use egui::{Align2, Color32, FontId, Painter, Pos2, Rect};
use egui::emath::Rot2;
use egui::epaint::TextShape;

pub trait RotatedText {
    fn rotated_text(
        &self,
        pos: Pos2,
        anchor: Align2,
        text: impl ToString,
        font_id: FontId,
        text_color: Color32,
        angle: f32,
    ) -> Rect;
}

impl RotatedText for Painter {
    fn rotated_text(
        &self,
        pos: Pos2,
        anchor: Align2,
        text: impl ToString,
        font_id: FontId,
        text_color: Color32,
        angle: f32,
    ) -> Rect {
        let galley = self.layout_no_wrap(text.to_string(), font_id, text_color);
        let rect = anchor.anchor_rect(Rect::from_min_size(pos, galley.size()));

        let half_size = galley.size() / 2.0;

        self.add(TextShape {
            angle,
            override_text_color: Some(text_color),
            ..TextShape::new(
                pos - Rot2::from_angle(angle) * (half_size + (anchor.to_sign() * half_size)),
                galley,
            )
        });

        rect
    }
}