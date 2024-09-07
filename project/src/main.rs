mod app;
mod calc;
mod styles;

use eframe::egui;
use app::CalculatorApp;

fn main() {
    let app = CalculatorApp::default();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 450.0)), // Set initial window size here
        ..Default::default()
    };
    eframe::run_native("Calculator", options, Box::new(|_cc| Box::new(app)));
}
