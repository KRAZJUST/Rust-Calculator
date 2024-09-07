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
}
