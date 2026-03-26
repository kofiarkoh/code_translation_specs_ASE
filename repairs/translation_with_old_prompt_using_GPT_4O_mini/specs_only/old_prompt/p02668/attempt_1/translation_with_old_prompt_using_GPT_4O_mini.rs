const N_MAX: usize = 1_048_576;

fn factorial(n: usize) -> u64 {
    let mut result = 1;
    for i in 1..=n {
        result = result.wrapping_mul(i as u64); // Use wrapping_mul to handle overflow
    }
    result
}

fn mod_inverse(a: u64, m: u64) -> u64 {
    let mut m0 = m;
    let mut y = 0;
    let mut x = 1;

    if m == 1 {
        return 0;
    }

    let mut a = a % m;

    while a > 1 {
        let q = a / m0;
        let t = m0;

        m0 = a % m0;
        a = t;
        t = y;

        y = x.wrapping_sub(q.wrapping_mul(y));
        x = t;
    }

    if x < 0 {
        x = x.wrapping_add(m);
    }

    x
}

fn solve(n: usize, m: usize) -> i32 {
    if n > N_MAX || m > N_MAX {
        return 1; // Return 1 on failure due to invalid input
    }

    let mut frac = vec![0; n + 1];
    let mut invf = vec![0; n + 1];
    let mut dp = vec![0; m + 1];
    let mut dq = vec![0; m + 1];

    // Precompute factorials and their inverses
    for i in 0..=n {
        frac[i] = factorial(i);
        invf[i] = mod_inverse(frac[i], 1_000_000_007); // Using a large prime for modular arithmetic
    }

    // Dynamic programming to fill dp table
    dp[0] = 1; // Base case
    for i in 1..=m {
        for j in 0..=n {
            if j >= i {
                dp[j] = (dp[j].wrapping_add(dp[j - i].wrapping_mul(frac[i]) % 1_000_000_007)) % 1_000_000_007;
            }
        }
    }

    // Output the result
    println!("{}", dp[n]);
    0 // Return 0 on success
}

fn main() {
    let n: usize = 10; // Example input, replace with actual input handling
    let m: usize = 5;  // Example input, replace with actual input handling
    let result = solve(n, m);
    std::process::exit(result as i32);
}
