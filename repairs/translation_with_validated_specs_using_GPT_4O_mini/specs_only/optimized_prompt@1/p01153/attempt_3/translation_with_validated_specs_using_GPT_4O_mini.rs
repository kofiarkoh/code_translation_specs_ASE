use std::io::{self, BufRead};

fn main() {
    let test_cases = read_input(); // Read the input as specified
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
                dp[start][end] = array[start]; // Base case: single element
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

// Function to read input from stdin
fn read_input() -> Vec<TestCase> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let t: usize = lines.next().unwrap().parse().unwrap(); // Read number of test cases
    let mut test_cases = Vec::new();

    for _ in 0..t {
        let n: usize = lines.next().unwrap().parse().unwrap(); // Read number of elements
        let array: Vec<i32> = lines.next().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect(); // Read the array of integers
        test_cases.push(TestCase { n, array });
    }

    test_cases
}
