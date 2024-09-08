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
        .fill(egui::Color32::from_rgb(45, 45, 45))
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
        .fill(egui::Color32::from_rgb(181, 84, 0))
        .rounding(egui::Rounding::same(5.0))
        .stroke(egui::Stroke::new(1.0, egui::Color32::BLACK))
}

/*
* Function to create custom style for the equation background
*/
pub fn draw_equation_background(ui: &mut egui::Ui, display: &str, previous_equation: &str) {
    let available_width = ui.available_width();
    let height_history: f32 = 45.0;
    let height = 75.0;

    // Space for the previous equation
    let (rect1, _) = ui.allocate_exact_size(egui::vec2(available_width, height_history), egui::Sense::hover());
    let background_color = egui::Color32::from_rgb(35, 35, 35);

    ui.painter().rect_filled(rect1, egui::Rounding::same(10.0), background_color);

    let mut result_size = 20.0;
    if display.len() >= 20 && display.len() < 30 {
        result_size = 16.0;
    }
    else if display.len() >= 30 && display.len() < 39 {
        result_size = 10.0;
    }

    let equation_text = egui::RichText::new(previous_equation)
        .size(result_size)
        .color(egui::Color32::GRAY);

    ui.painter().text(
        rect1.left_center(),
        egui::Align2::LEFT_CENTER,
        equation_text.text(),
        egui::FontId::proportional(result_size),
        egui::Color32::GRAY,
    );

    // Space for the current display (result or equation)
    let (rect2, _) = ui.allocate_exact_size(egui::vec2(available_width, height), egui::Sense::hover());
    let background_color2 = egui::Color32::from_rgb(30, 30, 30);

    ui.painter().rect_filled(rect2, egui::Rounding::same(10.0), background_color2);

    let mut font_size = 32.0;

    if display.len() < 14 {
        font_size = 32.0;
    }
    else if display.len() >= 14 && display.len() < 22 {
        font_size = 22.0;
    }
    else if display.len() >= 22 && display.len() < 39 {
        font_size = 12.0;
    }
    else {
        font_size = 12.0;
    }
    

    let display_text = egui::RichText::new(display)
        .size(font_size)
        .color(egui::Color32::WHITE);

    ui.painter().text(
        rect2.left_center(),
        egui::Align2::LEFT_CENTER,
        display_text.text(),
        egui::FontId::proportional(font_size),
        egui::Color32::WHITE,
    );
}

