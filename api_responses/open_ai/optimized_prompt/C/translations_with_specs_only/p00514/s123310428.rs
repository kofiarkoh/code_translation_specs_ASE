fn main() {
    // Read input values for n, m, and r
    let (n, m, r) = read_input();

    // Adjust r by subtracting the product of n and m
    let adjusted_r = r - n * m;

    // Determine the output based on the value of adjusted_r
    let output = if adjusted_r < 0 {
        0 // If r < 0, output is 0
    } else if adjusted_r == 0 {
        1 // If r == 0, output is 1
    } else {
        // Calculate the binomial coefficient (n + adjusted_r - 1)C(n - 1)
        binomial_coefficient(n + adjusted_r - 1, n - 1)
    };

    // Print the output
    println!("{}", output);
}

// Function to read input values for n, m, and r
fn read_input() -> (usize, usize, usize) {
    // Assuming valid input as per the specification
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<usize> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    (nums[0], nums[1], nums[2])
}

// Function to calculate the binomial coefficient (n choose k)
fn binomial_coefficient(n: usize, k: usize) -> usize {
    if k > n {
        return 0; // If k > n, the coefficient is 0
    }
    if k == 0 || k == n {
        return 1; // Base cases for binomial coefficient
    }

    // Use an array to represent large integers
    let mut result = vec![0; n + 1];
    result[0] = 1; // C(n, 0) = 1

    // Calculate the binomial coefficient using dynamic programming
    for i in 1..=n {
        for j in (1..=i).rev() {
            result[j] += result[j - 1]; // Update the array for C(i, j)
        }
    }

    result[k] // Return the binomial coefficient C(n, k)
}
