//! rustyCalculator — calculadora desktop em Rust puro (egui/eframe).
//!
//! Rode com: `cargo run`

mod app;
mod calculator;

use app::CalculatorApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([340.0, 480.0])
            .with_min_inner_size([300.0, 420.0])
            .with_title("rustyCalculator"),
        ..Default::default()
    };

    eframe::run_native(
        "rustyCalculator",
        options,
        Box::new(|_cc| Ok(Box::new(CalculatorApp::default()))),
    )
}
