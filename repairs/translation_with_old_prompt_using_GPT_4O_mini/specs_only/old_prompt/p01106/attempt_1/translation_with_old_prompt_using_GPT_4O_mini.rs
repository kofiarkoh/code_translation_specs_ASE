use std::io::{self, Write};

fn fold_ribbon(n: i32, i: i64, j: i64) {
    // Ensure n is greater than 0 as per the specification
    if n <= 0 {
        return; // No output for invalid n
    }

    let mut result = String::with_capacity(n as usize);
    let mut current_index = i;

    // Generate the folding pattern based on the binary representation
    for fold in 0..n {
        // Determine the fold direction based on the current index
        if (current_index >> (n - fold - 1)) & 1 == 0 {
            result.push('L'); // Left fold
        } else {
            result.push('R'); // Right fold
        }
    }

    // Output the resulting folding pattern
    println!("{}", result);
}

fn main() {
    // Example usage: reading from standard input
    let n = 5; // Number of folds
    let i = 0; // First index
    let j = 0; // Second index (not used in the current logic)

    fold_ribbon(n, i, j);
}
