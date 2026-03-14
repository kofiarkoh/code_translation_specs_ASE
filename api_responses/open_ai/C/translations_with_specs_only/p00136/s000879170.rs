use std::io::{self, Write};

fn main() {
    // Read the number of values to process
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the first input as a positive integer
    let count: usize = match input.trim().parse() {
        Ok(n) if n > 0 => n,
        _ => {
            eprintln!("Error: The first input must be a positive integer.");
            return;
        }
    };

    // Initialize counts for each category
    let mut categories = [0; 6];

    // Read the subsequent floating-point values
    for _ in 0..count {
        let mut value_input = String::new();
        io::stdin().read_line(&mut value_input).expect("Failed to read line");

        // Parse the input as a floating-point number
        match value_input.trim().parse::<f64>() {
            Ok(value) => {
                // Categorize the value into predefined ranges
                if value < 0.0 {
                    categories[0] += 1; // Category 1: (-∞, 0)
                } else if value < 10.0 {
                    categories[1] += 1; // Category 2: [0, 10)
                } else if value < 20.0 {
                    categories[2] += 1; // Category 3: [10, 20)
                } else if value < 30.0 {
                    categories[3] += 1; // Category 4: [20, 30)
                } else if value < 40.0 {
                    categories[4] += 1; // Category 5: [30, 40)
                } else {
                    categories[5] += 1; // Category 6: [40, ∞)
                }
            }
            Err(_) => {
                eprintln!("Error: Invalid floating-point number.");
                return;
            }
        }
    }

    // Output the histogram
    for count in categories.iter() {
        let stars = "*".repeat(*count);
        println!("{}", stars);
    }
}
