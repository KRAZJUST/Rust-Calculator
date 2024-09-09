use eframe::egui::{self, CentralPanel, Ui};
use eframe::{App, Frame};
use crate::styles::{draw_equation_background, result_button, styled_button};

#[derive(Default)]
pub struct CalculatorApp {
    pub display: String,
    pub result: Option<f64>,
    pub calculate: String,
    pub previous_equation: String,
}

impl App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        
        // Handle keyboard input
        if ctx.input(|i| i.key_pressed(egui::Key::Num0)){
            self.handle_button_press("0");
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::Num1)){
            self.handle_button_press("1");
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::Num2)){
            self.handle_button_press("2");
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::Num3)){
            self.handle_button_press("3");
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::Num4)){
            self.handle_button_press("4");
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::Num5)){
            self.handle_button_press("5");
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::Num6)){
            self.handle_button_press("6");
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::Num7)){
            self.handle_button_press("7");
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::Num8)){
            self.handle_button_press("8");
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::Num9)){
            self.handle_button_press("9");
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::Minus)){
            self.handle_button_press("-");
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::Plus)){
            self.handle_button_press("+");
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::Slash)){
            self.handle_button_press("/");
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::X)){
            self.handle_button_press("*");
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::Enter)){
            self.calculate();
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::Escape)){
            self.clear();
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::C)){
            self.clear();
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::Equals)){
            self.calculate();
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::Period)){
            self.handle_button_press(".");
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::Backspace)){
            self.handle_remove();
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::OpenBracket)){
            self.handle_button_press("(");
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::CloseBracket)){
            self.handle_button_press(")");
        }
        else if ctx.input(|i| i.key_pressed(egui::Key::E)) {
            self.handle_button_press("e");
        }
         

        CentralPanel::default().show(ctx, |ui| {
            // Draw background for the result and equation history
            draw_equation_background(ui, &self.display, &self.previous_equation);
            // Display buttons
            self.ui_buttons(ui);
        });
    }
}

impl CalculatorApp {
    /*
     * Function to add buttons into the Ui and handle them being pressed
     */
    fn ui_buttons(&mut self, ui: &mut Ui) {
        
        ui.horizontal(|ui| {
            if ui.add(styled_button("Clear")).clicked() {
                self.clear();
            }
            if ui.add(styled_button("(")).clicked() {
                self.handle_button_press("(");
            }
            if ui.add(styled_button(")")).clicked() {
                self.handle_button_press(")");
            }
            if ui.add(styled_button("e")).clicked() {
                self.handle_button_press("e");
            }
            if ui.add(styled_button("π")).clicked() {
                self.handle_button_press("pi");
            }
        });

        ui.horizontal(|ui| {
            if ui.add(styled_button("7")).clicked() {
                self.handle_button_press("7");
            }
            if ui.add(styled_button("8")).clicked() {
                self.handle_button_press("8");
            }
            if ui.add(styled_button("9")).clicked() {
                self.handle_button_press("9");
            }
            if ui.add(styled_button("÷")).clicked() {
                self.handle_button_press("/");
            }
            if ui.add(styled_button("√")).clicked() {
                self.handle_button_press("sqrt(")
            }
        });

        ui.horizontal(|ui| {
            if ui.add(styled_button("4")).clicked() {
                self.handle_button_press("4");
            }
            if ui.add(styled_button("5")).clicked() {
                self.handle_button_press("5");
            }
            if ui.add(styled_button("6")).clicked() {
                self.handle_button_press("6");
            }
            if ui.add(styled_button("×")).clicked() {
                self.handle_button_press("*");
            }
            if ui.add(styled_button("x²")).clicked() {
                self.handle_button_press("^2");
            }
        });

        ui.horizontal(|ui| {
            if ui.add(styled_button("1")).clicked() {
                self.handle_button_press("1");
            }
            if ui.add(styled_button("2")).clicked() {
                self.handle_button_press("2");
            }
            if ui.add(styled_button("3")).clicked() {
                self.handle_button_press("3");
            }
            if ui.add(styled_button("-")).clicked() {
                self.handle_button_press("-");
            }
        });

        ui.horizontal(|ui| {
            if ui.add(styled_button("0")).clicked() {
                self.handle_button_press("0");
            }
            if ui.add(styled_button(".")).clicked() {
                self.handle_button_press(".");
            }
            if ui.add(styled_button("mod")).clicked() {
                self.handle_button_press("%");
            }
            if ui.add(styled_button("+")).clicked() {
                self.handle_button_press("+");
            }
        });

        // Add space to move the result button up
        ui.add_space(-105.0);

        // Create the result button spanning two rows
        ui.horizontal(|ui| {
            ui.add_space(232.0);

            if ui.add(result_button("=")).clicked() {
                self.calculate();
            }
        });
    }
}