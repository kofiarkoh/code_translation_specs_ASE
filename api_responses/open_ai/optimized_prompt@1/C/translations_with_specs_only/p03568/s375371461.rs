fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line for N
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0), // Default to 0 if parsing fails
        _ => return, // Exit if no input is provided
    };

    // Initialize count of even integers
    let mut even_count = 0;

    // Read the next N integers
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(num) = line.trim().parse::<i32>() {
                // Check if the number is even
                if num % 2 == 0 {
                    even_count += 1; // Increment even count
                }
            }
            // If parsing fails, the behavior is not defined as per the specification
        }
    }

    // Calculate 3^N and 2^(count of even integers)
    let power_of_three = 3_i64.pow(n as u32); // 3^N
    let product_of_two = 2_i64.pow(even_count as u32); // 2^(count of even integers)

    // Compute the difference
    let result = power_of_three - product_of_two;

    // Output the result
    println!("{}", result);
}
