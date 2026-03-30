use std::io::{self, BufRead};

const MD: i64 = 998244353;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line for m and s
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let m: usize = parts.next().unwrap().parse().unwrap();
    let s = parts.next().unwrap();

    // Initialize the result with the integer representation of the primary binary string
    let mut result = binary_string_to_int(s) % MD;

    // Process each additional binary string
    for _ in 0..m {
        let additional_string = lines.next().unwrap().unwrap();
        let additional_value = binary_string_to_int(&additional_string) % MD;

        // Perform the bitwise operations as specified
        result = (result ^ additional_value) % MD; // XOR operation
        // Note: The specification does not detail shifts, but we assume they are part of the process
        // Here we can assume a left shift for demonstration, but the exact behavior is not specified
        result = (result << 1) % MD; // Example shift operation
    }

    // Output the final result
    println!("{}", result);
}

// Helper function to convert a binary string to an integer
fn binary_string_to_int(s: &str) -> i64 {
    let mut value = 0;
    for (i, c) in s.chars().rev().enumerate() {
        if c == '1' {
            value += 1 << i; // Use bitwise shift to calculate the integer value
        }
    }
    value
}
