fn count_distinct_subsequences(t: &str, b: &str) -> i64 {
    const MOD: i64 = 1_000_000_007;

    let n = t.len();
    let m = b.len();

    // Create a 2D array initialized to 0
    let mut a = vec![vec![0; m + 1]; n + 1];

    // Base case: An empty string b can be formed by any prefix of t (including empty prefix)
    for i in 0..=n {
        a[i][0] = 1; // There's one way to form an empty subsequence
    }

    // Fill the 2D array according to the dynamic programming approach
    for i in 1..=n {
        for j in 1..=m {
            // If characters match, we can either use this character or not
            if t.chars().nth(i - 1) == b.chars().nth(j - 1) {
                a[i][j] = (a[i - 1][j - 1] + a[i - 1][j]) % MOD;
            } else {
                // If they don't match, we can only skip the character from t
                a[i][j] = a[i - 1][j] % MOD;
            }
        }
    }

    // The result is in the bottom-right cell of the array
    a[n][m]
}

fn main() {
    let t = "rabbbit"; // Example input for t
    let b = "rabbit";  // Example input for b

    // Call the function and print the result
    let result = count_distinct_subsequences(t, b);
    println!("{}", result);
}
