use eframe::egui::{self, CentralPanel, Ui};
use eframe::{App, Frame};
use crate::styles::{draw_equation_background, result_button, styled_button};

#[derive(Default)]
pub struct CalculatorApp {
    pub display: String,
    pub input1: f64,
    pub input2: f64,
    pub operator: Option<char>,
    pub result: Option<f64>,
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
            self.display.clear();
            self.input1 = 0.0;
            self.input2 = 0.0;
            self.operator = None;
            self.result = None;
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
                self.handle_operator('/');
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
                self.handle_operator('*');
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
                self.handle_operator('-');
            }
        });

        ui.horizontal(|ui| {
            if ui.add(styled_button("0")).clicked() {
                self.handle_button_press("0");
            }
            if ui.add(styled_button(".")).clicked() {
                
            }
            if ui.add(styled_button("%")).clicked() {

            }
            if ui.add(styled_button("+")).clicked() {
                self.handle_operator('+');
            }
            if ui.add(result_button("=")).clicked() {
                self.calculate();
            }
        });
    }
}