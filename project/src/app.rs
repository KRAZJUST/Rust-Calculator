use eframe::egui::{self, CentralPanel, Ui};
use eframe::{App, Frame};
use crate::styles::{draw_equation_background, result_button, styled_button};

#[derive(Default)]
pub struct CalculatorApp {
    pub display: String,
    pub result: Option<f64>,
    pub calculate: String,
}

impl App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Calculator");

            draw_equation_background(ui, &self.display);

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
        if ui.add(styled_button("Clear")).clicked() {
            self.clear();
        }

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
            if ui.add(styled_button("%")).clicked() {
                self.convert_to_perct();
            }
            if ui.add(styled_button("+")).clicked() {
                self.handle_button_press("+");
            }
            if ui.add(result_button("=")).clicked() {
                self.calculate();
            }
        });
    }
}