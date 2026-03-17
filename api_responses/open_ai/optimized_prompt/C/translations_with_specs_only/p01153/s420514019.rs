fn main() {
    // Read the number of test cases
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let test_cases: usize = input.trim().parse().unwrap();

    for _ in 0..test_cases {
        // Read the size of the array
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let size: usize = input.trim().parse().unwrap();

        // Read the array elements
        let mut array = Vec::new();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        for num in input.trim().split_whitespace() {
            array.push(num.parse::<i32>().unwrap());
        }

        // Compute the maximum value obtainable from the array
        let max_value = compute_max_value(&array);
        println!("{}", max_value);
    }
}

// Function to compute the maximum value obtainable from the circular array
fn compute_max_value(array: &[i32]) -> i32 {
    let n = array.len();
    let mut dp = vec![vec![0; n]; n];

    // Fill the dynamic programming table
    for length in 1..=n {
        for start in 0..n {
            let end = (start + length - 1) % n; // Circular indexing
            if length == 1 {
                dp[start][end] = array[start]; // Base case
            } else {
                // Calculate the maximum value for the current subarray
                for k in 0..length - 1 {
                    let mid = (start + k) % n;
                    let next = (mid + 1) % n;
                    dp[start][end] = dp[start][end].max(dp[start][mid] + dp[next][end]);
                }
            }
        }
    }

    // The maximum value obtainable from the entire array
    let mut max_value = 0;
    for i in 0..n {
        max_value = max_value.max(dp[i][(i + n - 1) % n]);
    }
    max_value
}
