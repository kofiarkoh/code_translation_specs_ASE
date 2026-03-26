use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

fn main() {
    // Read input
    let stdin = io::stdin();
    let n: usize = stdin.lock().lines().next().unwrap().unwrap().parse().unwrap();

    // Precompute factorials and their modular inverses
    let mut fact = vec![1; n + 1];
    let mut invfact = vec![1; n + 1];

    for i in 2..=n {
        fact[i] = (fact[i - 1] * i as i64) % MOD;
    }

    // Compute inverses using Fermat's Little Theorem
    invfact[n] = mod_inverse(fact[n]);
    for i in (1..n).rev() {
        invfact[i] = (invfact[i + 1] * (i + 1) as i64) % MOD;
    }
    invfact[0] = 1; // Inverse of 0! is 1

    // Calculate the final result based on combinations
    let mut result = 0;
    for k in 0..=n {
        let comb = (fact[n] * invfact[k] % MOD) * invfact[n - k] % MOD;
        result = (result + comb) % MOD;
    }

    // Output the result
    println!("{}", result);
}

// Function to compute modular inverse using Fermat's Little Theorem
fn mod_inverse(x: i64) -> i64 {
    let mut result = 1;
    let mut exp = MOD - 2; // Since MOD is prime
    let mut base = x % MOD;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % MOD;
        }
        base = (base * base) % MOD;
        exp /= 2;
    }
    result
}
