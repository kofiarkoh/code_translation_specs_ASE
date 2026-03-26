use std::io::{self, BufRead};

fn main() {
    // Read the number of integers (N)
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for N and handle potential parsing errors
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.parse().unwrap_or(0), // Default to 0 on parse error
        _ => 0, // Default to 0 if no input is provided
    };

    // Ensure N is within the valid range
    if n < 1 || n > 100 {
        println!("0"); // Output 0 if N is out of bounds
        return;
    }

    // Initialize the array to hold the integers
    let mut a: Vec<i32> = Vec::with_capacity(n);

    // Read the integers into the array and handle potential parsing errors
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(num) = line.parse::<i32>() {
                a.push(num);
            }
        }
    }

    // Count consecutive duplicates
    let mut count = 0;
    for i in 1..n {
        if a[i] == a[i - 1] {
            count += 1; // Increment count for each pair of consecutive duplicates
        }
    }

    // Print the count of consecutive duplicates
    println!("{}", count);
}
