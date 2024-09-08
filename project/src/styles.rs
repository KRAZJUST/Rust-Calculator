use eframe::egui;
use eframe::egui::RichText;

/*
* Function to create custom styled button
*/
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

/*
* Function to create custom styled '=' button
*/
pub fn result_button(label: &str) -> egui::Button {
    egui::Button::new(RichText::new(label).size(18.0).strong())
        .min_size(egui::vec2(50.0, 101.0))
        .fill(egui::Color32::from_rgb(225, 135, 40))
        .rounding(egui::Rounding::same(5.0))
        .stroke(egui::Stroke::new(1.0, egui::Color32::BLACK))
}

/*
* Function to create custom style for the equation background
*/
pub fn draw_equation_background(ui: &mut egui::Ui, display: &str) {
    let available_width = ui.available_width();
    let height = 75.0; // Height for the display area

    // Allocate space for the equation display
    let (rect, _) = ui.allocate_exact_size(egui::vec2(available_width, height), egui::Sense::hover());

    let background_color = egui::Color32::from_rgb(30, 30, 30);

    // Draw the rectangle with the background color
    ui.painter().rect_filled(rect, egui::Rounding::same(10.0), background_color);

    // Display the equation/result text
    let display_text = egui::RichText::new(display)
        .size(32.0)
        .color(egui::Color32::WHITE);

    // Manually align the text to the left inside the rectangle
    ui.painter().text(
        // Align to the left and center vertically
        rect.left_center(),
        // Align text to the left and center vertically
        egui::Align2::LEFT_CENTER,
        display_text.text(),
        egui::FontId::proportional(32.0),
        egui::Color32::WHITE,
    );
}

