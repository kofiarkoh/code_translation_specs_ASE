fn compute_value(n: i64, m: i64) -> i64 {
    // Preconditions: m must be >= 1
    assert!(m >= 1, "m must be greater than or equal to 1");

    if m == 1 {
        return 0; // If m equals 1, the output is 0.
    }

    let half_m = m / 2; // Calculate m/2

    if half_m < n {
        return half_m; // If m/2 < n, output is m/2.
    } else if half_m >= n && half_m < n + 2 {
        return n; // If n <= m/2 < n + 2, output is n.
    } else {
        // Otherwise, output is n + (m/2 - n) / 2.
        return n + (half_m - n) / 2;
    }
}

fn main() {
    // Example usage of compute_value function
    let n = 5; // Example input
    let m = 10; // Example input
    let result = compute_value(n, m);
    println!("The computed value is: {}", result);
}
