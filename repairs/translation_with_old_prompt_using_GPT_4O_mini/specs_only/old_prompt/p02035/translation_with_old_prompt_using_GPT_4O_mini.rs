use std::io;

fn main() {
    // Read input values for n and m
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut inputs = input.split_whitespace();
    
    let n: usize = inputs.next().unwrap().parse().expect("Invalid input for n");
    let m: usize = inputs.next().unwrap().parse().expect("Invalid input for m");

    // Preconditions check
    assert!(n >= 0, "n must be non-negative");
    assert!(m > 1, "m must be greater than 1");

    // Initialize factorial and inverse factorial arrays
    let mut fact = vec![1; n + 1];
    let mut invfact = vec![1; n + 1];

    // Compute factorials modulo m
    for i in 1..=n {
        fact[i] = (fact[i - 1] * i) % m;
    }

    // Compute modular inverses of factorials using Fermat's Little Theorem
    invfact[n] = mod_inverse(fact[n], m);
    for i in (1..n).rev() {
        invfact[i] = (invfact[i + 1] * (i + 1)) % m;
    }

    // Compute the result based on the combinatorial expression
    let mut result = 0;
    for k in 0..=n {
        let comb = (fact[n] * invfact[k] % m) * invfact[n - k] % m;
        result = (result + comb * mod_pow(2, k, m)) % m;
    }

    // Output the result
    println!("{}", result);
}

// Function to compute modular exponentiation
fn mod_pow(base: usize, exp: usize, modulus: usize) -> usize {
    let mut result = 1;
    let mut b = base % modulus;
    let mut e = exp;

    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % modulus;
        }
        b = (b * b) % modulus;
        e /= 2;
    }
    result
}

// Function to compute modular inverse using Fermat's Little Theorem
fn mod_inverse(a: usize, m: usize) -> usize {
    let mut result = 1;
    let mut exponent = m - 2;
    let mut base = a;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % m;
        }
        base = (base * base) % m;
        exponent /= 2;
    }
    result
}
