use crate::app::CalculatorApp;
use meval::eval_str;

impl CalculatorApp {
    /*
    * Function to add operators and operands into the equation
    */
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
            "pi" => {
                self.display.push_str("π");
            }
            "^2" => {
                self.display.push_str("²");
            }
            "%" => {
                self.display.push_str("mod");
            }
            _ => {
                self.display.push_str(label);
            }
        }
    }

    /*
    * Function to remove last digit from equation
    */
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
                self.display = "Malformed exp.".to_string();
                self.calculate = "Error".to_string();
            }
        }
    }

    /*
    * Function to clear the result and reset the state of calculator
    */
    pub fn clear(&mut self) {
        self.display.clear(); 
        self.calculate.clear();
        self.result = None;
    }

}
