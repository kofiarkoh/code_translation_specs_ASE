// This program evaluates three binary integer inputs (0 or 1) 
// and determines a state, which is either "Open" or "Close".

fn evaluate_state(input1: i32, input2: i32, input3: i32) -> String {
    // Ensure inputs are within the expected range (0 or 1)
    if !is_valid_input(input1) || !is_valid_input(input2) || !is_valid_input(input3) {
        // If inputs are invalid, we could return an error or panic, 
        // but the specification does not define error handling.
        // For now, we will return "Close" as a default state.
        return "Close".to_string();
    }

    // Determine the state based on the specified combinations
    match (input1, input2, input3) {
        (1, 0, 0) | (0, 1, 0) | (0, 0, 0) => "Close".to_string(),
        (1, 1, 0) | (0, 0, 1) => "Open".to_string(),
        _ => "Close".to_string(), // Default case, not specified but ensures a return
    }
}

// Helper function to validate input
fn is_valid_input(input: i32) -> bool {
    input == 0 || input == 1
}

// Example usage
fn main() {
    let state = evaluate_state(1, 0, 0);
    println!("{}", state); // Output: Close
}
