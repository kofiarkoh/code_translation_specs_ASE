fn main() {
    let test_cases = read_input(); // Assume this function reads the input as specified
    for case in test_cases {
        let result = compute_max_value(case);
        println!("{}", result);
    }
}

// Structure to hold each test case
struct TestCase {
    n: usize, // Number of elements in the array
    array: Vec<i32>, // The array of integers
}

// Function to compute the maximum value based on the specified logic
fn compute_max_value(test_case: TestCase) -> i32 {
    let n = test_case.n;
    let array = test_case.array;

    // Dynamic programming table to store computed values
    let mut dp = vec![vec![0; n]; n];

    // Fill the DP table based on the relationships defined in the specification
    for length in 1..=n {
        for start in 0..n {
            let end = (start + length - 1) % n; // Circular indexing
            if length == 1 {
                dp[start][end] = array[start].abs(); // Base case: single element
            } else {
                // Compute the value based on the differences and previously computed values
                let diff = (array[start] - array[end]).abs();
                dp[start][end] = diff + dp[start][(end + n - 1) % n].max(dp[(start + 1) % n][end]);
            }
        }
    }

    // Find the maximum value in the DP table
    let mut max_value = 0;
    for i in 0..n {
        for j in 0..n {
            max_value = max_value.max(dp[i][j]);
        }
    }

    max_value
}

// Placeholder function to simulate input reading
fn read_input() -> Vec<TestCase> {
    // This function should be implemented to read the input as specified
    vec![]
}
