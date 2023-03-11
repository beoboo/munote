use eframe::{
    egui,
    epaint::{
        Color32,
        Shape,
        Stroke,
        text::FontData,
    },
};
use egui::{Align2, Painter, pos2, Pos2, Rect, TextStyle, vec2, Visuals};
use tracing::info;

use munote::score::Score;
use munote::tag::Tag;
use munote::tag_id::TagId;

use crate::context::Context;
use crate::symbol::Symbol;

mod context;
mod symbol;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    tracing::info!("Starting");

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };

    eframe::run_native("Munote", options, Box::new(|cc| Box::new(App::new(cc))))
}

struct App {}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);

        Self {}
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "noto".to_owned(),
        FontData::from_static(include_bytes!("../assets/noto.ttf")),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "noto".to_owned());

    ctx.set_fonts(fonts);
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::light());

        egui::CentralPanel::default().show(ctx, |ui| {
            let color = if ui.visuals().dark_mode {
                Color32::from_additive_luminance(196)
            } else {
                Color32::from_black_alpha(240)
            };

            let size = 64.0;
            let padding = 50.0;
            let margin_top = 50.0;
            let width = ui.available_width();
            let height = ui.available_height();

            let mut font_id = TextStyle::Body.resolve(ui.style());
            font_id.size = size;

            let mut context = Context::new(
                pos2(padding, margin_top + padding),
                width - 2.0 * padding,
                height - 2.0 * padding,
                color,
                size,
                font_id,
            );

            let score = Score::parse("[ \\clef c d e f g a b c2 ]")
                .expect("Cannot parse score");

            // let score = Score::parse("[ \\clef a ]")
            //     .expect("Cannot parse score");

            render_score(score, ui.painter(), &mut context);
        });
    }
}

fn render_score(score: Score, painter: &Painter, context: &mut Context) {
    painter.rect(Rect::from_min_max(
        context.origin,
        context.origin + vec2(context.width, context.height),
    ), 0.0, Color32::TRANSPARENT, Stroke::new(1.0, Color32::RED));

    for (_, staff) in score.staffs {
        for voice in staff.voices {
            render_staff(voice.staff, painter, context);

            let clef = Symbol::from_clef(&Tag::from_id(TagId::Clef), context);
            render_symbol(clef, painter, context);

            for event in voice.events {
                if let Some(symbol) = Symbol::from_event(&event, context) {
                    info!("{symbol:?}");
                    render_symbol(symbol, painter, context);
                }
            }
        }
    }
}

fn render_staff(id: u8, painter: &Painter, context: &mut Context) {
    let mut lines = vec![];
    let pos = context.position;
    let width = context.width;

    for i in 0..5 {
        let y = pos.y + i as f32 * context.font_size / 4.0;

        lines.push(Shape::line_segment(
            [Pos2::new(pos.x, y), Pos2::new(pos.x + width, y)],
            Stroke::new(1.0, context.color),
        ));
    }

    painter.extend(lines);
    context.position.y += context.font_size / 2.0 - 2.0;
}

fn render_symbol(symbol: Symbol, painter: &Painter, context: &mut Context) {
    let pos = &mut context.position;
    let symbol_pos = symbol.pos;

    let rect = painter.text(
        pos2(pos.x + symbol_pos.x, pos.y + symbol_pos.y),
        Align2::CENTER_CENTER,
        symbol.glyph,
        context.font_id.clone(),
        symbol.color,
    );

    // if context.draw_bounds {
    painter.rect(rect, 0.0, Color32::TRANSPARENT, Stroke::new(1.0, Color32::RED));
    // }

    pos.x += rect.width();
}
//
// // Clef
// let pos = painter.text(
//     pos2(padding + size / 2.0, margin_top + size / 2.0 - 2.0),
//     Align2::CENTER_CENTER,
//     Symbols::get("G CLEF").unwrap(),
//     font_id.clone(),
//     color,
// );
//
// // Note (A)
// painter.text(
//     pos2(pos.right() + size / 2.0, margin_top + size + size / 8.0),
//     Align2::CENTER_BOTTOM,
//     Symbols::get("NOTEHEAD BLACK").unwrap(),
//     font_id.clone(),
//     color,
// );
//
// // Bar ending
// painter.text(
//     pos2(width - padding - 8.0, margin_top + size / 2.0 - 2.0),
//     Align2::CENTER_CENTER,
//     Symbols::get("FINAL BARLINE").unwrap(),
//     font_id,
//     color,
// );
