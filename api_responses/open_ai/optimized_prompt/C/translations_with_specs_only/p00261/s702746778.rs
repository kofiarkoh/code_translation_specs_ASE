use std::io::{self, BufRead};

fn main() {
    // Define the mapping of cities based on the specification.
    // For this example, let's assume a simple mapping where:
    // '0' -> City A, '1' -> City B, '2' -> City C, ..., '9' -> City J
    let city_mapping = vec![
        "City A", "City B", "City C", "City D", "City E",
        "City F", "City G", "City H", "City I", "City J"
    ];

    // Define the target city we are checking against.
    let target_city = "City C"; // Example target city

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let input = line.unwrap(); // Unwrap the result; in a real application, handle errors appropriately

        // Terminate processing if the input starts with '#'
        if input.starts_with('#') {
            break;
        }

        // Check if the input contains only digits
        if !input.chars().all(char::is_numeric) {
            // If invalid characters are found, terminate processing for this string
            continue; // No output for this string as per specification
        }

        // Start processing the path
        let mut current_city_index = 0; // Start at the first city (City A)

        for c in input.chars() {
            let digit = c.to_digit(10).unwrap() as usize; // Convert char to digit
            if digit < city_mapping.len() {
                current_city_index = digit; // Transition to the next city based on the digit
            } else {
                // If the digit is out of bounds, we stop processing this string
                break;
            }
        }

        // Determine if the final city reached is the target city
        if city_mapping[current_city_index] == target_city {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
