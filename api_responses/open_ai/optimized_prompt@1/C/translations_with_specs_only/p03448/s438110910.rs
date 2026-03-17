fn count_combinations(a: u32, b: u32, c: u32, x: u32) -> u32 {
    let mut count = 0; // Initialize the count of valid combinations to zero

    // Iterate through all possible values of the three variables
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                // Check if the weighted sum equals the target value
                if i + j + k == x {
                    count += 1; // Increment count for each valid combination found
                }
            }
        }
    }

    count // Return the total count of valid combinations
}

fn main() {
    // Example usage: replace with actual input values as needed
    let a = 3; // Upper limit for the first variable
    let b = 3; // Upper limit for the second variable
    let c = 3; // Upper limit for the third variable
    let x = 6; // Target sum

    let result = count_combinations(a, b, c, x);
    println!("{}", result); // Output the result
}
