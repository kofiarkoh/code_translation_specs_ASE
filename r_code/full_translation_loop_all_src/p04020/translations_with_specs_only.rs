use std::io::{self, BufRead};

fn main() {
    // Read the number of integers
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read N
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.parse().unwrap_or(0),
        _ => return, // Exit if reading fails
    };
    
    // Ensure N is within the specified bounds
    if n < 1 || n > 100_000 {
        return; // Exit if N is out of bounds
    }

    let mut sum: u64 = 0; // Using u64 to ensure non-negative output

    // Vector to store the integers
    let mut integers = Vec::with_capacity(n);

    // Read the integers and compute the sum
    for _ in 0..n {
        let value: i64 = match lines.next() {
            Some(Ok(line)) => match line.parse() {
                Ok(v) => v,
                Err(_) => return, // Exit if parsing fails
            },
            _ => return, // Exit if reading fails
        };
        integers.push(value);
    }

    // Calculate the sum based on the specification
    for i in 0..n {
        let half = (integers[i] / 2) as u64; // Compute half of the integer
        sum += half; // Add half to the sum

        // Adjust based on the parity of adjacent integers
        if i > 0 {
            if (integers[i] % 2) != (integers[i - 1] % 2) {
                sum += 1; // Adjust if current and previous integers have different parity
            }
        }
    }

    // Output the computed value
    println!("{}", sum);
}
``` 

// Note: The previous implementation did not correctly handle the input format where multiple integers are provided on a single line. The following code has been adjusted to read all integers from a single line after reading N. 

```rust
use std::io::{self, BufRead};

fn main() {
    // Read the number of integers
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read N
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.parse().unwrap_or(0),
        _ => return, // Exit if reading fails
    };
    
    // Ensure N is within the specified bounds
    if n < 1 || n > 100_000 {
        return; // Exit if N is out of bounds
    }

    let mut sum: u64 = 0; // Using u64 to ensure non-negative output

    // Read the integers from the next line
    let integers: Vec<i64> = match lines.next() {
        Some(Ok(line)) => line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect(),
        _ => return, // Exit if reading fails
    };

    // Ensure we read exactly N integers
    if integers.len() != n {
        return; // Exit if the number of integers does not match N
    }

    // Calculate the sum based on the specification
    for i in 0..n {
        let half = (integers[i] / 2) as u64; // Compute half of the integer
        sum += half; // Add half to the sum

        // Adjust based on the parity of adjacent integers
        if i > 0 {
            if (integers[i] % 2) != (integers[i - 1] % 2) {
                sum += 1; // Adjust if current and previous integers have different parity
            }
        }
    }

    // Output the computed value
    println!("{}", sum);
}
