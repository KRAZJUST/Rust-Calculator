use eframe::egui::{self, CentralPanel, Ui};
use eframe::{App, Frame};

#[derive(Default)]
struct CalculatorApp {
    display: String,
    input1: f64,
    input2: f64,
    operator: Option<char>,
    result: Option<f64>,
}

impl CalculatorApp {
    fn handle_button_press(&mut self, label: &str) {
        if self.operator.is_some() {
            self.display.push_str(label);
        } else {
            self.display.push_str(label);
        }
    }

    fn handle_operator(&mut self, operator: char) {
        self.operator = Some(operator);
        if let Ok(num) = self.display.parse::<f64>() {
            self.input1 = num;
            self.display.clear();
        }
    }

    fn calculate(&mut self) {
        if let Ok(num) = self.display.parse::<f64>() {
            self.input2 = num;
        }

        if let Some(op) = self.operator {
            self.result = Some(match op {
                '+' => self.input1 + self.input2,
                '-' => self.input1 - self.input2,
                '*' => self.input1 * self.input2,
                '/' => {
                    if self.input2 == 0.0 {
                        return; // Avoid division by zero
                    }
                    self.input1 / self.input2
                }
                _ => 0.0,
            });
        }

        if let Some(res) = self.result {
            self.display = res.to_string();
        }
    }
}

impl App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Calculator");

            ui.label(&self.display);

            // Display buttons
            self.ui_buttons(ui);
        });
    }
}

impl CalculatorApp {
    fn ui_buttons(&mut self, ui: &mut Ui) {
        // Create the number buttons and operator buttons
        ui.horizontal(|ui| {
            if ui.button("1").clicked() {
                self.handle_button_press("1");
            }
            if ui.button("2").clicked() {
                self.handle_button_press("2");
            }
            if ui.button("3").clicked() {
                self.handle_button_press("3");
            }
        });
        ui.horizontal(|ui| {
            if ui.button("4").clicked() {
                self.handle_button_press("4");
            }
            if ui.button("5").clicked() {
                self.handle_button_press("5");
            }
            if ui.button("6").clicked() {
                self.handle_button_press("6");
            }
        });
        ui.horizontal(|ui| {
            if ui.button("7").clicked() {
                self.handle_button_press("7");
            }
            if ui.button("8").clicked() {
                self.handle_button_press("8");
            }
            if ui.button("9").clicked() {
                self.handle_button_press("9");
            }
        });
        ui.horizontal(|ui| {
            if ui.button("0").clicked() {
                self.handle_button_press("0");
            }
        });

        // Operator buttons
        ui.horizontal(|ui| {
            if ui.button("+").clicked() {
                self.handle_operator('+');
            }
            if ui.button("-").clicked() {
                self.handle_operator('-');
            }
            if ui.button("*").clicked() {
                self.handle_operator('*');
            }
            if ui.button("/").clicked() {
                self.handle_operator('/');
            }
        });

        if ui.button("=").clicked() {
            self.calculate();
        }

        if ui.button("Clear").clicked() {
            self.display.clear();
            self.input1 = 0.0;
            self.input2 = 0.0;
            self.operator = None;
            self.result = None;
        }
    }
}

fn main() {
    let app = CalculatorApp::default();
    let options = eframe::NativeOptions::default();
    eframe::run_native("Calculator", options, Box::new(|_cc| Box::new(app)));
}
