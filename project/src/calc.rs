use crate::app::CalculatorApp;
use meval::eval_str;

impl CalculatorApp {
    // Handle button presses (numbers and operators)
    pub fn handle_button_press(&mut self, label: &str) {
        self.display.push_str(label);
    }

    // Calculate the result when '=' is pressed
    pub fn calculate(&mut self) {
        // Try to evaluate the entire expression stored in `display`
        match eval_str(&self.display) {
            Ok(result) => {
                // Display the result
                self.display = result.to_string();
            }
            Err(_) => {
                // Display an error message if the evaluation fails
                self.display = "Error".to_string(); 
            }
        }
    }

    // Clear the display and reset state
    pub fn clear(&mut self) {
        self.display.clear(); 
        self.result = None;
    }

    // TODO: fix this function 
    // Convert the result to a percentage 
    pub fn convert_to_perct(&mut self) {
        if let Some(result) = self.result {
            // Convert to percentage by multiplying by 100
            let percentage = result * 100.0;

            // Update the display with the percentage value and a '%' symbol
            self.display = format!("{:.2}%", percentage);
            
            // Update the result field to hold the percentage value for further operations
            self.result = Some(percentage);
        }
    }
}
