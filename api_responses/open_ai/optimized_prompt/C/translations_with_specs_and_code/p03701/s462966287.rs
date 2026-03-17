use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of integers
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Preconditions: n must be positive
    if n == 0 {
        return; // Undefined behavior as per spec
    }

    let mut sum = 0;
    let mut min = 101; // Initialize min to a value greater than any valid input
    let mut has_non_zero = false;

    // Read the integers and calculate the sum and minimum
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        sum += value;

        // Check divisibility by 10
        if value % 10 == 0 {
            // Set to 101 if divisible by 10
            continue; // Skip adding this value to min
        }

        // Update minimum if the current value is less than the current minimum
        if min > value {
            min = value;
            has_non_zero = true; // Track if we have a non-zero minimum
        }
    }

    let mut ans = sum;

    // Postconditions: Calculate the final answer based on the conditions
    if sum % 10 == 0 {
        if !has_non_zero {
            ans = 0; // If no non-zero minimum was found
        } else {
            ans -= min; // Subtract the minimum non-zero value
        }
    }

    // Output the result
    println!("{}", ans);
}
