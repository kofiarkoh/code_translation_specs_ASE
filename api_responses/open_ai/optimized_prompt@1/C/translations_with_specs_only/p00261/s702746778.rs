use std::io::{self, BufRead};

fn main() {
    // Define the mapping of cities as a static array.
    // Each index represents a city, and the value at that index represents the next city for each digit transition.
    // For example, if city 0 transitions to city 1 on input '0', then mapping[0][0] = 1.
    let mapping: [[Option<usize>; 10]; 10] = [
        [Some(1), Some(2), None, None, None, None, None, None, None, None],
        [Some(3), None, None, None, None, None, None, None, None, None],
        [Some(4), None, None, None, None, None, None, None, None, None],
        [Some(5), None, None, None, None, None, None, None, None, None],
        [Some(6), None, None, None, None, None, None, None, None, None],
        [Some(7), None, None, None, None, None, None, None, None, None],
        [Some(8), None, None, None, None, None, None, None, None, None],
        [Some(9), None, None, None, None, None, None, None, None, None],
        [Some(0), None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None, None],
    ];

    // Define the target city we want to check against.
    const TARGET_CITY: usize = 5;

    // Read input from standard input line by line.
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let input = line.unwrap();

        // Termination condition: if the input starts with '#', break the loop.
        if input.starts_with('#') {
            break;
        }

        // Validate input length
        if input.len() > 109 {
            println!("No");
            continue;
        }

        // Start processing the input string
        let mut current_city = 0; // Start at city 0
        let mut valid = true; // Flag to track if the transitions are valid

        for ch in input.chars() {
            // Convert character to digit
            if let Some(digit) = ch.to_digit(10) {
                let digit = digit as usize;

                // Check if the transition is valid
                if let Some(next_city) = mapping[current_city][digit] {
                    current_city = next_city; // Update to the next city
                } else {
                    valid = false; // Invalid transition
                    break;
                }
            } else {
                valid = false; // Invalid character
                break;
            }
        }

        // Output "Yes" if the final city is the target city and the transitions were valid
        if valid && current_city == TARGET_CITY {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
