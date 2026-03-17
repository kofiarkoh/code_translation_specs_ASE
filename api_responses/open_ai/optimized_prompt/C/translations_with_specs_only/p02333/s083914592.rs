// Define the constant for the modulo operation
const MODULO: i64 = 1_000_000_007;

// Function to compute factorial modulo MODULO
fn factorial(n: usize) -> i64 {
    let mut fact = vec![1; n + 1];
    for i in 1..=n {
        fact[i] = (fact[i - 1] * i as i64) % MODULO;
    }
    fact[n]
}

// Function to compute modular inverse using Fermat's Little Theorem
fn mod_inverse(a: i64) -> i64 {
    let mut m = MODULO - 2; // MODULO is prime, so we can use Fermat's theorem
    let mut result = 1;
    let mut base = a;

    while m > 0 {
        if m % 2 == 1 {
            result = (result * base) % MODULO;
        }
        base = (base * base) % MODULO;
        m /= 2;
    }
    result
}

// Function to compute the binomial coefficient C(n, m) modulo MODULO
fn binomial_coefficient(n: usize, m: usize) -> i64 {
    if m > n {
        return 0; // C(n, m) is 0 if m > n
    }
    let numerator = factorial(n);
    let denominator = (factorial(m) * factorial(n - m)) % MODULO;
    (numerator * mod_inverse(denominator)) % MODULO
}

// Main function to compute the result based on inputs n and m
fn compute_expression(n: i64, m: i64) -> i64 {
    // Check preconditions
    if n < 0 || m < 0 || m > 2000 {
        return 0; // Return 0 if inputs are invalid
    }
    // Compute the binomial coefficient C(n, m)
    binomial_coefficient(n as usize, m as usize)
}

// Entry point for the program
fn main() {
    // Example usage (replace with actual input handling as needed)
    let n = 5; // Example input for n
    let m = 2; // Example input for m
    let result = compute_expression(n, m);
    println!("{}", result); // Output the result
}
