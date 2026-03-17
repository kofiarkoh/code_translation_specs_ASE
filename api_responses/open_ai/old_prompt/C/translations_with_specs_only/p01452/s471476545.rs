fn main() {
    // Read input values for x, y, and k
    let (x, y, k) = read_input();

    // Calculate the maximum value for the memoization table
    let n = x + y + 2 * k;
    const MAX_SIZE: usize = 220_001;

    // Ensure n does not exceed the bounds of the memoization array
    if n > MAX_SIZE {
        println!("0");
        return;
    }

    // Initialize the memoization table for factorial values
    let mut memo = vec![0; MAX_SIZE];
    const MOD: i64 = 1_000_000_007; // Define a modulus for calculations

    // Precompute factorials modulo MOD
    memo[0] = 1; // 0! = 1
    for i in 1..=n {
        memo[i] = (memo[i - 1] * i as i64) % MOD;
    }

    // Calculate the result using combinatorial logic
    let result = if k > x || k > y {
        0 // If k exceeds x or y, return 0 as it's invalid
    } else {
        // Calculate combinations C(x + k, k) * C(y + k, k) % MOD
        let comb_x = combination(x + k, k, &memo, MOD);
        let comb_y = combination(y + k, k, &memo, MOD);
        (comb_x * comb_y) % MOD
    };

    // Print the result
    println!("{}", result);
}

// Function to read input values
fn read_input() -> (i32, i32, i32) {
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    let nums: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    (nums[0], nums[1], nums[2])
}

// Function to calculate combinations C(n, r) using memoization
fn combination(n: i32, r: i32, memo: &[i64], mod_val: i64) -> i64 {
    if r > n || r < 0 {
        return 0; // Invalid combination
    }
    let numerator = memo[n as usize];
    let denominator = (memo[r as usize] * memo[(n - r) as usize]) % mod_val;
    // Calculate modular inverse of denominator using Fermat's Little Theorem
    let inv_denominator = mod_inverse(denominator, mod_val);
    (numerator * inv_denominator) % mod_val
}

// Function to calculate modular inverse using Fermat's Little Theorem
fn mod_inverse(a: i64, mod_val: i64) -> i64 {
    let mut m = mod_val;
    let mut x0 = 0;
    let mut x1 = 1;
    let mut a = a % mod_val;

    while a > 1 {
        let q = a / m;
        let t = m;
        m = a % m;
        a = t;
        let t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }

    if x1 < 0 {
        x1 += mod_val;
    }
    x1
}
