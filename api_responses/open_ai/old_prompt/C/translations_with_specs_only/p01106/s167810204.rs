fn fold_ribbon(n: i32, i: i64, j: i64) -> String {
    // Ensure n is greater than 0 as per the specification
    if n <= 0 {
        return String::new(); // No output if n is 0 or negative
    }

    let mut result = String::with_capacity(n as usize); // Preallocate space for the output string

    // The folding pattern is determined by the binary representation of the indices
    for k in 0..n {
        // Calculate the current index based on the number of folds
        let index = (i + j) >> k; // Right shift to determine the current fold
        // Determine if the current fold is 'L' or 'R'
        if index & 1 == 0 {
            result.push('L'); // Even index results in 'L'
        } else {
            result.push('R'); // Odd index results in 'R'
        }
    }

    result // Return the resulting folding pattern
}

fn main() {
    // Example usage
    let n = 5; // Number of folds
    let i = 0; // First index
    let j = 1; // Second index
    let folding_pattern = fold_ribbon(n, i, j);
    println!("{}", folding_pattern); // Output the folding pattern
}
