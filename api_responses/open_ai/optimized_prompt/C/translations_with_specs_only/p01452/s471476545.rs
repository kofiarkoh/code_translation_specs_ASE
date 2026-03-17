// Define the predefined constant for the modulo operation
const MODULO_CONSTANT: i32 = 1_000_000_007;

// Function to compute the combinatorial value
fn compute_combinatorial(a: i32, b: i32, c: i32) -> i32 {
    // Check preconditions: all inputs must be non-negative
    if a < 0 || b < 0 || c < 0 {
        return 0; // Return zero for negative inputs
    }
    
    // Check if the first two integers are less than the third
    if a < c || b < c {
        return 0; // Return zero if the first two integers are less than the third
    }

    // Check the sum condition
    if a + b + 2 * c > MODULO_CONSTANT {
        return 0; // Return zero if the sum exceeds the predefined limit
    }

    // Compute the combinatorial value (a choose c) * (b choose c)
    let comb_a_c = combinations(a, c);
    let comb_b_c = combinations(b, c);

    // Return the result modulo the predefined constant
    (comb_a_c * comb_b_c) % MODULO_CONSTANT
}

// Function to compute combinations (n choose k)
fn combinations(n: i32, k: i32) -> i32 {
    if k > n {
        return 0; // Return zero if k is greater than n
    }
    if k == 0 || k == n {
        return 1; // Base cases for combinations
    }

    // Calculate n! / (k! * (n - k)!)
    let mut num = 1; // Numerator
    let mut denom = 1; // Denominator

    for i in 0..k {
        num = (num * (n - i)) % MODULO_CONSTANT; // Compute numerator
        denom = (denom * (i + 1)) % MODULO_CONSTANT; // Compute denominator
    }

    // Compute modular inverse of denom using Fermat's Little Theorem
    let denom_inv = mod_inverse(denom, MODULO_CONSTANT);
    
    // Return the result of the combination modulo the predefined constant
    (num * denom_inv) % MODULO_CONSTANT
}

// Function to compute modular inverse using Fermat's Little Theorem
fn mod_inverse(a: i32, m: i32) -> i32 {
    // a^(m-2) mod m
    let mut result = 1;
    let mut exp = m - 2;
    let mut base = a;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % m; // Multiply result by base if exp is odd
        }
        base = (base * base) % m; // Square the base
        exp /= 2; // Divide exp by 2
    }

    result
}

// Example usage
fn main() {
    let a = 5;
    let b = 3;
    let c = 2;
    let result = compute_combinatorial(a, b, c);
    println!("{}", result); // Output the result
}
