use std::io::{self, BufRead};

fn main() {
    // Define the state transition map as per the specification.
    // The map is assumed to be predefined and remains unchanged.
    let map: [usize; 6] = [1, 2, 3, 4, 5, 0]; // Example mapping, adjust as needed.

    // Read input from standard input until termination character '#'.
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Handle potential I/O errors.
        let path = line.expect("Failed to read line");

        // Initialize the current state to 0 (the starting state).
        let mut current_state = 0;

        // Process each character in the input string.
        for c in path.chars() {
            // Check for the termination character.
            if c == '#' {
                break; // Stop processing on termination character.
            }

            // Convert character to a digit and check if it's valid.
            if let Some(digit) = c.to_digit(10) {
                let transition = digit as usize;
                if transition < map.len() {
                    // Update the current state based on the transition.
                    current_state = map[transition];
                } else {
                    // Invalid transition character, terminate processing early.
                    break;
                }
            } else {
                // Invalid character encountered, terminate processing early.
                break;
            }
        }

        // Output "Yes" if the final state is B (assumed to be state 1 here).
        // Adjust the condition based on the actual target state.
        if current_state == 1 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
