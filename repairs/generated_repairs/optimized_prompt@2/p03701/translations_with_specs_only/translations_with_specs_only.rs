fn main() {
    use std::io::{self, BufRead};

    // Read the number of integers from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line which indicates the number of integers
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => 0,
    };

    // Preconditions state that n must be a positive integer
    if n == 0 {
        // Undefined behavior for non-positive n, we simply return
        return;
    }

    let mut sum = 0;
    let mut min_value = i32::MAX; // Initialize min_value to the maximum possible i32
    let mut valid_input = false; // Track if we have valid input

    // Read the subsequent n integers
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(value) = line.trim().parse::<i32>() {
                // Maintain the sum and track the minimum value
                sum += value;
                if value < min_value {
                    min_value = value;
                }
                valid_input = true; // Mark that we have valid input
            }
        }
    }

    // If no valid input was processed, we should return 0
    if !valid_input {
        println!("0");
        return;
    }

    // Calculate the modified sum based on the conditions
    // If the minimum value is divisible by 10, we subtract it from the sum
    if min_value % 10 == 0 {
        sum -= min_value; // Directly subtract min_value
    }

    // Output the modified sum
    println!("{}", sum);
}
