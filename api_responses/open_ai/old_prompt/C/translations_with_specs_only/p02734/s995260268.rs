fn count_ways(n: usize, s: usize, a: &[i32]) -> i64 {
    const MOD: i64 = 998244353;

    // Initialize a dp array with size (n+1) x (s+1)
    let mut dp = vec![vec![0; (s + 1) as usize]; (n + 1) as usize];
    
    // Base case: There's one way to achieve the sum of 0 (by choosing no elements)
    dp[0][0] = 1;

    // Fill the dp array
    for i in 1..=n {
        for j in 0..=s {
            // Carry forward the previous count
            dp[i][j] = dp[i - 1][j];

            // If the current element can be included
            if j as i32 - a[i - 1] >= 0 {
                dp[i][j] = (dp[i][j] + dp[i - 1][j as i32 - a[i - 1] as usize]) % MOD;
            }
        }
    }

    // The result is the number of ways to achieve the sum s using the first n elements
    dp[n][s] as i64
}

fn main() {
    // Example input
    let n = 3; // Number of integers
    let s = 4; // Target sum
    let a = vec![1, 2, 3]; // Array of integers

    // Calculate the number of ways and print the result
    let result = count_ways(n, s, &a);
    println!("{}", result);
}
