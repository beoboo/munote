pub trait DisplayEvent {
    fn duration(&self) -> f32;
    fn glyph(&self) -> char;
    fn adjust(&self) -> (f32, f32);
}