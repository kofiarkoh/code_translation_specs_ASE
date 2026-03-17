// Constants
const MOD: i64 = 1_000_000_007; // Define the modulus as per the specification
const MAX_M: usize = 2001; // Maximum value for m as per the specification

// Global arrays for factorial and inverse factorial
static mut FACT: [i64; MAX_M] = [0; MAX_M];
static mut INVFACT: [i64; MAX_M] = [0; MAX_M];

// Function to compute factorials and their modular inverses
fn makefact(m: usize) {
    unsafe {
        FACT[0] = 1; // 0! = 1
        for i in 1..=m {
            FACT[i] = (FACT[i - 1] * i as i64) % MOD; // Compute factorial
        }
        INVFACT[m] = mod_inverse(FACT[m]); // Compute inverse of m! using modular inverse
        for i in (1..=m).rev() {
            INVFACT[i - 1] = (INVFACT[i] * i as i64) % MOD; // Compute inverse factorial
        }
    }
}

// Function to compute modular inverse using Fermat's Little Theorem
fn mod_inverse(x: i64) -> i64 {
    let mut result = 1;
    let mut exp = MOD - 2; // Fermat's theorem exponent
    let mut base = x % MOD;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % MOD; // Multiply result by base if exp is odd
        }
        base = (base * base) % MOD; // Square the base
        exp /= 2; // Divide exponent by 2
    }
    result
}

// Function to compute binomial coefficient C(m, k)
fn binomial_coefficient(m: usize, k: usize) -> i64 {
    if k > m {
        return 0; // C(m, k) is 0 if k > m
    }
    unsafe {
        (FACT[m] * INVFACT[k] % MOD * INVFACT[m - k] % MOD) % MOD // C(m, k) = m! / (k! * (m-k)!)
    }
}

// Main function to compute the combinatorial sum
fn compute_sum(n: i64, m: usize) -> i64 {
    if n < 0 || m > 2000 {
        return 0; // Invalid input handling as per specification
    }
    unsafe {
        makefact(m); // Prepare factorials and inverse factorials
    }
    
    let mut result = 0; // Initialize result
    for k in 0..=m {
        let term = (binomial_coefficient(m, k) * mod_exp(k as i64, n)) % MOD; // Compute each term
        result = (result + term) % MOD; // Accumulate result
    }
    result
}

// Function for modular exponentiation
fn mod_exp(base: i64, exp: i64) -> i64 {
    let mut result = 1;
    let mut b = base % MOD;
    let mut e = exp;
    
    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % MOD; // Multiply result by base if exp is odd
        }
        b = (b * b) % MOD; // Square the base
        e /= 2; // Divide exponent by 2
    }
    result
}

// Entry point of the program
fn main() {
    let n: i64 = 5; // Example input for n
    let m: usize = 10; // Example input for m
    let result = compute_sum(n, m); // Compute the sum
    println!("{}", result); // Output the result
}
