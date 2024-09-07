mod app;
mod calc;
mod styles;

use eframe::egui::{self, ViewportBuilder};
use app::CalculatorApp;

fn main() {
    let app = CalculatorApp::default();
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([300.0, 400.0]),
        ..Default::default()
    };
    let _ = eframe::run_native("Calculator", options, Box::new(|_cc| Ok(Box::new(app))));
}
