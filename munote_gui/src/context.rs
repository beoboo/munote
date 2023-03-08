use egui::{Color32, FontId, Pos2};

pub struct Context {
    pub origin: Pos2,
    pub position: Pos2,
    pub width: f32,
    pub height: f32,
    pub color: Color32,
    pub font_size: f32,
    pub font_id: FontId,
}

impl Context {
    pub fn new(
        origin: Pos2,
        width: f32,
        height: f32,
        color: Color32,
        font_size: f32,
        font_id: FontId,
    ) -> Self {
        Self {
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