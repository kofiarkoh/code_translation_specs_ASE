fn main() {
    // Read the number of test cases
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let test_cases: usize = input.trim().parse().unwrap();

    for _ in 0..test_cases {
        // Read the size of the array
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let size: usize = input.trim().parse().unwrap();

        // Read the array of integers
        input.clear();
        let array: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Compute the maximum value based on the defined operations
        let max_value = compute_max_value(&array);
        println!("{}", max_value);
    }
}

// Function to compute the maximum value obtainable from the circular array
fn compute_max_value(array: &[i32]) -> i32 {
    let n = array.len();
    // Dynamic programming table to store maximum values
    let mut dp = vec![0; n];

    // Initialize the dp table based on the first element
    dp[0] = array[0];

    // Fill the dp table based on the defined operations
    for i in 1..n {
        // The maximum value at dp[i] is either taking the current element
        // or the maximum value from the previous elements
        dp[i] = array[i].max(dp[i - 1]);
    }

    // The maximum value in the dp table is the result
    *dp.iter().max().unwrap()
}
