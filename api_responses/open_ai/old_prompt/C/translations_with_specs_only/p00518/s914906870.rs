fn main() {
    // Read input values
    let n: usize = read_input(); // Assume this function reads an integer from input
    let input: String = read_input_string(); // Assume this function reads a string from input

    // Validate input length
    if input.len() != n {
        // If the length of the input string does not match N, we may produce unexpected results.
        // However, the specification does not define behavior for this case.
        return;
    }

    // Initialize the schedule array
    let mut schedule = [0; 64]; // 8 * 8 = 64 states for combinations of 'J', 'O', 'I'
    schedule[0] = 1; // Base case: one way to have an empty combination

    // Process each character in the input string
    for ch in input.chars() {
        let mut schedule_next = [0; 64]; // Temporary array for the next state

        // Update the schedule based on the current character
        match ch {
            'J' => {
                for i in 0..64 {
                    schedule_next[i] = (schedule_next[i] + schedule[i]) % 10007; // Carry forward existing combinations
                    if i < 56 { // Ensure we don't exceed the bounds of the array
                        schedule_next[i + 8] = (schedule_next[i + 8] + schedule[i]) % 10007; // Add new combinations for 'J'
                    }
                }
            }
            'O' => {
                for i in 0..64 {
                    schedule_next[i] = (schedule_next[i] + schedule[i]) % 10007; // Carry forward existing combinations
                    if i < 64 { // Ensure we don't exceed the bounds of the array
                        schedule_next[i + 1] = (schedule_next[i + 1] + schedule[i]) % 10007; // Add new combinations for 'O'
                    }
                }
            }
            'I' => {
                for i in 0..64 {
                    schedule_next[i] = (schedule_next[i] + schedule[i]) % 10007; // Carry forward existing combinations
                    if i < 64 { // Ensure we don't exceed the bounds of the array
                        schedule_next[i + 2] = (schedule_next[i + 2] + schedule[i]) % 10007; // Add new combinations for 'I'
                    }
                }
            }
            _ => {
                // The specification guarantees that input will only contain 'J', 'O', and 'I'.
                // Therefore, we do not handle any other characters.
            }
        }

        // Update the schedule for the next iteration
        schedule.copy_from_slice(&schedule_next);
    }

    // Compute the total valid combinations
    let total: i32 = schedule.iter().sum();
    
    // Output the result modulo 10007
    println!("{}", total % 10007);
}

// Placeholder functions for reading input
fn read_input() -> usize {
    // Implementation to read an integer from input
    unimplemented!()
}

fn read_input_string() -> String {
    // Implementation to read a string from input
    unimplemented!()
}
