fn main() {
    // Read input values for n and m
    let (n, m) = read_input();

    // Check preconditions: n and m must be non-negative
    if n < 0 || m < 0 {
        println!("{}", 0);
        return;
    }

    // Define the modulo constant
    const MOD: i64 = 1_000_000_007;

    // Initialize factorial and inverse factorial arrays
    let (factorial, inv_factorial) = compute_factorials_and_inverses(m, MOD);

    // Compute the result based on the mathematical expression
    let result = compute_expression(n, m, &factorial, &inv_factorial, MOD);

    // Output the result
    println!("{}", result);
}

// Function to read input values for n and m
fn read_input() -> (i64, i64) {
    // For simplicity, we assume input is provided in a valid format
    // In a real scenario, error handling would be necessary
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i64> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    (nums[0], nums[1])
}

// Function to compute factorials and their inverses modulo MOD
fn compute_factorials_and_inverses(m: i64, mod_val: i64) -> (Vec<i64>, Vec<i64>) {
    let mut factorial = vec![1; (m + 1) as usize];
    let mut inv_factorial = vec![1; (m + 1) as usize];

    for i in 1..=m {
        factorial[i as usize] = (factorial[(i - 1) as usize] * i) % mod_val;
    }

    // Compute inverse of factorial[m] using Fermat's Little Theorem
    inv_factorial[m as usize] = mod_exp(factorial[m as usize], mod_val - 2, mod_val);
    
    for i in (1..=m).rev() {
        inv_factorial[(i - 1) as usize] = (inv_factorial[i as usize] * i) % mod_val;
    }

    (factorial, inv_factorial)
}

// Function to compute modular exponentiation
fn mod_exp(base: i64, exp: i64, mod_val: i64) -> i64 {
    let mut result = 1;
    let mut b = base % mod_val;
    let mut e = exp;

    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % mod_val;
        }
        b = (b * b) % mod_val;
        e /= 2;
    }
    result
}

// Function to compute the final expression based on combinations and modular exponentiation
fn compute_expression(n: i64, m: i64, factorial: &[i64], inv_factorial: &[i64], mod_val: i64) -> i64 {
    let mut result = 0;

    for k in 0..=m {
        if k > n {
            break; // No need to compute combinations if k > n
        }
        let comb = (factorial[n] * inv_factorial[k] % mod_val * inv_factorial[n - k] % mod_val) % mod_val;
        let term = (comb * mod_exp(2, k, mod_val)) % mod_val;
        result = (result + term) % mod_val;
    }

    result
}
