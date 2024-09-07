use eframe::egui;
use eframe::egui::RichText;

pub fn styled_button(label: &str) -> egui::Button {
    egui::Button::new(RichText::new(label).size(18.0).strong())
        // Set minimum size
        .min_size(egui::vec2(50.0, 50.0))
        // Background color
        .fill(egui::Color32::from_rgb(59, 59, 59))
        // Rounded corners
        .rounding(egui::Rounding::same(5.0))
        // Border with black stroke
        .stroke(egui::Stroke::new(1.0, egui::Color32::BLACK))
}

pub fn result_button(label: &str) -> egui::Button {
    egui::Button::new(RichText::new(label).size(18.0).strong())
        .min_size(egui::vec2(50.0, 50.0))
        .fill(egui::Color32::from_rgb(225, 135, 40))
        .rounding(egui::Rounding::same(5.0))
        .stroke(egui::Stroke::new(1.0, egui::Color32::BLACK))
}
