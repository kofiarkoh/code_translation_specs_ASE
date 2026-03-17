fn main() {
    use std::io::{self, BufRead};

    // Read the number of integers from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line which indicates the number of integers
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => return, // Undefined behavior for non-positive input
    };

    // Ensure n is positive as per the specification
    if n == 0 {
        return; // Undefined behavior for non-positive input
    }

    let mut sum = 0;
    let mut min_value: Option<i32> = None;

    // Read the subsequent integers
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(value) = line.trim().parse::<i32>() {
                sum += value; // Maintain the sum of integers

                // Update the minimum value if applicable
                if value != 0 {
                    if let Some(min) = min_value {
                        if value < min {
                            min_value = Some(value);
                        }
                    } else {
                        min_value = Some(value);
                    }
                }
            }
            // If parsing fails, behavior is undefined as per the spec
        }
    }

    // Determine the output based on the conditions specified
    let output = match min_value {
        Some(min) if min > 0 => sum - min, // Subtract the smallest non-zero integer
        _ => sum, // Just return the total sum if no valid min found
    };

    // Ensure the output is non-negative as per the postconditions
    println!("{}", output);
}
