use std::io;

const MAX_SIZE: usize = 220_001;
const MOD: i64 = 1_000_000_007;

fn main() {
    let (x, y, k) = read_input();
    let result = calculate_combinations(x, y, k);
    println!("{}", result);
}

fn read_input() -> (i64, i64, i64) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i64> = input.split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();
    (nums[0], nums[1], nums[2])
}

fn calculate_combinations(x: i64, y: i64, k: i64) -> i64 {
    let n = x + y + 2 * k;
    if n < 0 || n as usize >= MAX_SIZE {
        return 0; // Out of bounds
    }

    let mut memo = vec![0; MAX_SIZE];
    memo[0] = 1; // 0! = 1

    // Precompute factorials modulo MOD
    for i in 1..=n {
        memo[i as usize] = (memo[(i - 1) as usize] * i) % MOD;
    }

    // Calculate the result using combinations
    let result = if k > x || k > y {
        0 // Invalid combination
    } else {
        let numerator = memo[n as usize];
        let denominator = (memo[k as usize] * memo[(n - k) as usize]) % MOD;
        (numerator * mod_inverse(denominator, MOD)) % MOD
    };

    result
}

// Function to compute modular inverse using Fermat's Little Theorem
fn mod_inverse(a: i64, m: i64) -> i64 {
    let mut m0 = m;
    let mut y = 0;
    let mut x = 1;

    if m == 1 {
        return 0;
    }

    let mut a = a % m;

    while a > 1 {
        let q = a / m0;
        let t = m0; // Store m0 in t

        m0 = a % m0;
        a = t; // Update a to previous m0
        let t = y; // Store y in t

        y = x - q * y; // Update y
        x = t; // Update x to previous y
    }

    if x < 0 {
        x += m;
    }

    x
}
