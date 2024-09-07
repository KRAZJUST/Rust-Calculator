use crate::app::CalculatorApp;

impl CalculatorApp {
    pub fn handle_button_press(&mut self, label: &str) {
        if self.operator.is_some() {
            self.display.push_str(label);
        } else {
            self.display.push_str(label);
        }
    }

    pub fn handle_operator(&mut self, operator: char) {
        self.operator = Some(operator);
        if let Ok(num) = self.display.parse::<f64>() {
            self.input1 = num;
            self.display.clear();
        }
    }

    pub fn calculate(&mut self) {
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