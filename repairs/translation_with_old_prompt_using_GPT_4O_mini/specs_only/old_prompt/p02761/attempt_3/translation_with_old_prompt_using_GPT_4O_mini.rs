use std::io::{self, Write};

fn construct_number(n: usize, m: usize, s: &[usize], c: &[usize]) -> i32 {
    // Create a vector to hold the digit values, initialized to -1 (unassigned)
    let mut digits = vec![-1; n];

    // Process the digit-value pairs
    for i in 0..m {
        let pos = s[i] as usize - 1; // Convert to 0-based index
        let value = c[i];

        // Check for conflicts: if the position is already assigned a value
        if pos < n {
            if digits[pos] != -1 && digits[pos] != value as i32 {
                return -1; // Conflict detected
            }
            digits[pos] = value as i32; // Assign the value
        }
    }

    // Handle the case where M is 0: default to 0 for all digits
    if m == 0 {
        return 0;
    }

    // Construct the final number
    let mut num = 0;
    for (i, &digit) in digits.iter().enumerate() {
        if digit == -1 {
            return -1; // If any digit is unassigned, return -1
        }
        // Ensure that the first digit is not zero unless it's the only digit
        if i == 0 && digit == 0 && n > 1 {
            return -1; // Invalid number with leading zero
        }
        num = num * 10 + digit; // Build the number
    }

    num
}

fn main() {
    // Example input reading (for demonstration purposes)
    let n = 3; // Number of digits
    let m = 2; // Number of digit-value pairs
    let s = vec![1, 3]; // Digit positions
    let c = vec![1, 5]; // Corresponding values

    // Construct the number based on the inputs
    let result = construct_number(n, m, &s, &c);

    // Output the result
    println!("{}", result);
}
