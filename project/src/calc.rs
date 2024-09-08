use crate::app::CalculatorApp;
use meval::eval_str;

impl CalculatorApp {
    /*
    * Function to add operators and operands into the equation
    */
    pub fn handle_button_press(&mut self, label: &str) {
        if self.display.len() > 39 {
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

    pub fn check_last_two(&mut self) -> bool {
        // Get the total number of characters
        let len = self.calculate.chars().count();
        
        // Ensure the string has at least two characters
        if len < 2 {
            return false;
        }
    
        // Get the second-to-last and last characters
        let mut chars = self.calculate.chars().rev();
    
        let last = chars.next().unwrap();
        let pre_last = chars.next().unwrap();
    
        // Check if the second-to-last is '^' and the last is '2'
        pre_last == '^' && last == '2'
    }

    /*
    * Function to remove last digit from equation
    */
    pub fn handle_remove(&mut self) {
        if self.check_last_two(){
            self.calculate.pop();
            self.calculate.pop();
            self.display.pop();
        } 
        else {
            self.calculate.pop();
            self.display.pop();
        }
    }

    /*
    * Function to calculate the result and limit the number of digits so it fits into the app window
    */
    pub fn calculate(&mut self) {
        // Try to evaluate the entire expression stored in `calculate`
        match eval_str(&self.calculate) {
            Ok(result) => {
                self.previous_equation = self.display.clone();

                // Check if the result is a int and format it accordnigly
                if result.fract() == 0.0 {
                    // Display as an integer
                    self.display = format!("{:.0}", result);
                }
                // Display as decimal
                else { 
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
