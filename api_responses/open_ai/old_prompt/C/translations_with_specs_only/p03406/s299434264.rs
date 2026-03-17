fn main() {
    // Example input values for testing
    let n = 5; // Size of the set
    let m = 3; // Number of constraints
    let aa = [1, 2, 3]; // Constraints array

    // Initialize the combinatorial calculation
    let result = calculate_combinations(n, m, &aa);
    
    // Print the result
    println!("{}", result);
}

const MD: i64 = 1_000_000_007; // Modulo value for the result

fn calculate_combinations(n: usize, m: usize, aa: &[i64]) -> i64 {
    // Validate input ranges
    assert!(n <= 16);
    assert!(m <= 16);
    for &a in aa {
        assert!(a >= 0 && a < (1 << n)); // Each element must be in the range [0, 2^n - 1]
    }

    let size = 1 << n; // 2^n
    let mut vv = vec![0; size]; // Array for storing valid combinations
    let mut ff = vec![1; n + 1]; // Factorial array
    let mut gg = vec![1; n + 1]; // Inverse factorial array
    let mut dp = vec![0; size]; // Dynamic programming array

    // Precompute factorials and inverse factorials
    for i in 1..=n {
        ff[i] = (ff[i - 1] * i as i64) % MD;
    }
    gg[n] = mod_inverse(ff[n], MD);
    for i in (1..n).rev() {
        gg[i] = (gg[i + 1] * (i + 1) as i64) % MD;
    }

    // Fill the vv array based on constraints
    for &a in aa {
        vv[a as usize] = 1; // Mark valid combinations
    }

    // Dynamic programming to calculate the result
    for i in 0..size {
        for j in 0..m {
            if vv[i] == 1 {
                dp[i] = (dp[i] + choose(n, j as i64, &ff, &gg)) % MD; // Use choose function
            }
        }
    }

    dp.iter().sum::<i64>() % MD // Return the final result
}

// Function to compute n choose k
fn choose(n: usize, k: i64, ff: &[i64], gg: &[i64]) -> i64 {
    if k > n as i64 {
        return 0; // If k > n, return 0
    }
    (ff[n] * gg[k as usize] % MD * gg[(n as i64 - k) as usize] % MD) % MD
}

// Function to compute modular inverse using Fermat's Little Theorem
fn mod_inverse(a: i64, m: i64) -> i64 {
    let mut x = 1;
    let mut y = 0;
    let mut m0 = m;
    let mut a = a % m;

    if a == 0 {
        return 0; // Inverse doesn't exist
    }

    while a > 1 {
        let q = a / m0; // Quotient
        let t = m0; // Store m0
        m0 = a % m0; // Update m0
        a = t; // Update a
        let t = x; // Store x
        x = y - q * x; // Update x
        y = t; // Update y
    }

    if y < 0 {
        y += m; // Ensure y is positive
    }

    y // Return the modular inverse
}
