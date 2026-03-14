use std::io::{self, Write};

fn main() {
    loop {
        // Read input from the user
        let mut input = String::new();
        print!("Enter two integers and the number of iterations (space-separated): ");
        io::stdout().flush().unwrap(); // Ensure prompt is displayed

        if io::stdin().read_line(&mut input).is_err() {
            // If reading input fails, terminate without output
            return;
        }

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 3 {
            // If input does not contain exactly three integers, terminate without output
            return;
        }

        // Parse the integers
        let a: i32 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => return, // Terminate on parse error
        };
        let b: i32 = match parts[1].parse() {
            Ok(num) => num,
            Err(_) => return, // Terminate on parse error
        };
        let iterations: i32 = match parts[2].parse() {
            Ok(num) => num,
            Err(_) => return, // Terminate on parse error
        };

        // Check for division by zero
        if b == 0 {
            // Division by zero leads to undefined behavior; terminate without output
            return;
        }

        // Initialize the cumulative sum
        let mut sum = 0;
        let mut current_a = a;

        // Perform the specified number of iterations
        for _ in 0..iterations {
            // Perform the division and multiplication
            let division_result = current_a / b;
            sum += division_result;
            current_a = division_result * b; // Adjust current_a based on the division result
        }

        // Output the cumulative sum
        println!("{}", sum);
    }
}
