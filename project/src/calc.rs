use crate::app::CalculatorApp;
use meval::eval_str;

impl CalculatorApp {
    // Handle button presses (numbers and operators)
    pub fn handle_button_press(&mut self, label: &str) {
        if self.calculate.len() >= 13 && (label != "*" || label != "+" || label != "-" || label != "/"){
            return;
        }
        // Push label into calculate to keep equation to solve
        self.calculate.push_str(label);

        // Push label into display to display the input equation
        match label {
            "*" => {
                self.display.push_str("×");
            }
            "/" => {
                self.display.push_str("÷");
            }
            _ => {
                self.display.push_str(label);
            }
        }
    }

    pub fn handle_remove(&mut self) {
        self.calculate.pop();
        self.display.pop();
    }

    /*
    * Function to calculate the result and limit the number of digits so it fits into the app window
    */
    pub fn calculate(&mut self) {
        // Try to evaluate the entire expression stored in `calculate`
        match eval_str(&self.calculate) {
            Ok(result) => {
                let result_str = result.to_string();
                let digits_before_decimal = result_str.split('.').next().unwrap_or("").len();

                // If the total digits exceed 13, switch to scientific notation
                if digits_before_decimal > 13 {
                    // Display in scientific notation if too large
                    self.display = format!("{:.6e}", result); // 6 decimal places in scientific notation
                } else {
                    // Calculate max decimal places to limit to 13 total digits
                    let max_decimal_places = if digits_before_decimal >= 13 {
                        0 // No decimal places if the integer part is too long
                    } else {
                        13 - digits_before_decimal
                    };

                    // Display the result normally with limited decimal places
                    self.display = format!("{:.1$}", result, max_decimal_places);
                }

                // Store the result in the calculate field for further operations
                self.calculate = self.display.clone();
            }
            Err(_) => {
                // Display an error message if the evaluation fails
                self.display = "Error".to_string();
                self.calculate = "Error".to_string();
            }
        }
    }

    // Clear the display and reset state
    pub fn clear(&mut self) {
        self.display.clear(); 
        self.calculate.clear();
        self.result = None;
    }

    // TODO: fix this function 
    // Convert the result to a percentage 
    pub fn convert_to_perct(&mut self) {
        if let Some(result) = self.result {
            // Convert to percentage by multiplying by 100
            let percentage = result * 100.0;
            
            // Update the display with the percentage value and a '%' symbol
            self.display = format!("{:.2}%", percentage.to_string());
            
            // Update the result field to hold the percentage value for further operations
            self.result = Some(percentage);
        }
    }
}
