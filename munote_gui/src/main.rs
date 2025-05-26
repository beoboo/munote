use eframe::{
    egui,
    epaint::{
        Color32,
        text::FontData,
    },
};
use egui::{pos2, TextStyle, Visuals};

use munote::score::Score;
use munote::visitor::VisitorPtr;

use crate::drawing_context::DrawingContext;
use crate::playback_context::PlaybackContext;

mod drawing_context;
mod symbol;
mod playback_context;
mod ui;

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

struct App {
    score: Score
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);

        // let score = Score::parse("[ \\clef c d e f g a b c2 ]")
        //     .expect("Cannot parse score");
        //
        let score = Score::parse("[ \\clef _/2 g/8. g/16 a/4 g c2 b1/2 ]")
            .expect("Cannot parse score");

        // let score = Score::parse("[ \\clef c/4. ]")
        //     .expect("Cannot parse score");

        Self {
            score
        }
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
            ui.horizontal(|ui| {
                if ui.button("Play").clicked() {
                    self.play()
                }
            });

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
            let painter = ui.painter().clone();

            let context = Box::new(DrawingContext::new(
                painter,
                pos2(padding, margin_top + padding),
                width - 2.0 * padding,
                height - 2.0 * padding,
                color,
                size,
                font_id,
            ));

            self.score.visit(VisitorPtr::new(context));
        });
    }

}

impl App {
    fn play(&self) {
        let tempo = 60.0 * 1000.0 / 130.0;

        println!("Tempo: {tempo}");

        let context = PlaybackContext::new(tempo)
            .expect("Cannot create MIDI output");
        self.score.visit(VisitorPtr::new(Box::new(context)));
    }
}
