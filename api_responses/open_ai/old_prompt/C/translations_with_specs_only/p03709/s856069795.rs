fn solve(n: usize, a: Vec<i64>, b: Vec<i64>) -> i32 {
    // Check if n is 0, handle this edge case gracefully
    if n == 0 {
        println!("Yes 0");
        return 0; // Return 0 on success
    }

    // Initialize dynamic programming tables
    let mut dp = vec![0; n];
    let mut ss = vec![0; n];

    // Populate dp and ss based on the input pairs
    for i in 0..n {
        dp[i] = a[i] + b[i]; // Example computation, adjust as needed
        ss[i] = a[i] - b[i]; // Example computation, adjust as needed
    }

    // Example logic to determine the result
    let result = if dp.iter().any(|&x| x > 0) {
        "Yes"
    } else {
        "No"
    };

    // Print the result
    println!("{} {}", result, dp.iter().sum::<i64>());
    
    0 // Return 0 on success
}

fn main() {
    // Example usage
    let n = 3; // Number of pairs
    let a = vec![1, 2, 3]; // Example input for a
    let b = vec![4, 5, 6]; // Example input for b

    // Call the solve function
    let _ = solve(n, a, b);
}
