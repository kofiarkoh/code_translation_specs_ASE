fn count_combinations(a: usize, b: usize, c: usize, x: usize) -> usize {
    let mut count = 0; // Initialize the count of valid combinations to zero

    // Iterate through all possible values of the three variables
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                // Check if the weighted sum equals the target value
                if i + j + k == x {
                    count += 1; // Increment count for each valid combination
                }
            }
        }
    }

    count // Return the total count of valid combinations
}

fn main() {
    // Example usage
    let a = 2; // Upper limit for the first variable
    let b = 2; // Upper limit for the second variable
    let c = 2; // Upper limit for the third variable
    let x = 3; // Target sum

    let result = count_combinations(a, b, c, x);
    println!("{}", result); // Output the result
}
