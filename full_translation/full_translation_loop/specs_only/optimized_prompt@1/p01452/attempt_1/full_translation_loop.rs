const MODULO: i32 = 1_000_000_007; // Define a constant for the modulo operation

fn factorial(n: i32) -> i32 {
    let mut result = 1;
    for i in 1..=n {
        result = (result * i) % MODULO; // Ensure we stay within the modulo
    }
    result
}

fn mod_inverse(a: i32) -> i32 {
    // Using Fermat's Little Theorem for modular inverse
    let mut m = MODULO - 2; // MODULO is prime, so we can use this
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

fn combination(n: i32, k: i32) -> i32 {
    if k > n || n < 0 || k < 0 {
        return 0; // Return 0 for invalid combinations
    }
    let numerator = factorial(n);
    let denominator = (factorial(k) * factorial(n - k)) % MODULO;
    (numerator * mod_inverse(denominator)) % MODULO // Calculate C(n, k) % MODULO
}

fn compute_combinatorial_value(a: i32, b: i32, c: i32) -> i32 {
    if a < 0 || b < 0 || c < 0 || a < b {
        return 0; // Return 0 for invalid inputs
    }
    combination(a, b) // Compute the combinatorial value
}

fn main() {
    // Example usage
    let a = 5; // Example input
    let b = 2; // Example input
    let c = 0; // Example input (not used in computation)
    let result = compute_combinatorial_value(a, b, c);
    println!("{}", result); // Output the result
}
