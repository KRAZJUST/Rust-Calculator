use crate::app::CalculatorApp;

impl CalculatorApp {
    // Handle button presses (numbers)
    pub fn handle_button_press(&mut self, label: &str) {
        self.display.push_str(label); // Append numbers to the display
    }

    // Handle operators (+, -, *, /)
    pub fn handle_operator(&mut self, operator: char) {
        if self.operator.is_none() {
            if let Ok(num) = self.display.parse::<f64>() {
                self.input1 = num;
                self.operator = Some(operator);
                self.display.push(operator); // Append the operator to the display
            }
        }
    }

    // Calculate the result when '=' is pressed
    pub fn calculate(&mut self) {
        if let Some(op) = self.operator {
            // Try to parse the second number (input2)
            let equation: Vec<&str> = self.display.split(op).collect();

            if equation.len() == 2 {
                if let Ok(num) = equation[1].parse::<f64>() {
                    self.input2 = num;
                }

                // Perform the calculation based on the operator
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

                // Update the display with the result
                if let Some(res) = self.result {
                    self.display = format!("{}", res);
                }

                // Reset for the next calculation
                self.operator = None;
                self.input1 = 0.0;
                self.input2 = 0.0;
            }
        }
    }
}
