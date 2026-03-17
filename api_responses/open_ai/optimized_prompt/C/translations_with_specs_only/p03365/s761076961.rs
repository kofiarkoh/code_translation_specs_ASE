// Constants
const MOD: u64 = 1_000_000_007;
const MAX_N: usize = 1_000_000;

// Function to compute factorials and their modular inverses
fn precompute_factorials_and_inverses() -> (Vec<u64>, Vec<u64>) {
    let mut fact = vec![1; MAX_N + 1];
    let mut invfact = vec![1; MAX_N + 1];

    // Compute all factorials % MOD
    for i in 2..=MAX_N {
        fact[i] = (fact[i - 1] * i as u64) % MOD;
    }

    // Compute inverses using Fermat's Little Theorem
    invfact[MAX_N] = mod_inverse(fact[MAX_N]);
    for i in (1..MAX_N).rev() {
        invfact[i] = (invfact[i + 1] * (i + 1) as u64) % MOD;
    }

    (fact, invfact)
}

// Function to compute modular inverse using Fermat's Little Theorem
fn mod_inverse(x: u64) -> u64 {
    let mut result = 1;
    let mut exponent = MOD - 2; // MOD is prime, so we can use Fermat's theorem
    let mut base = x;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % MOD;
        }
        base = (base * base) % MOD;
        exponent /= 2;
    }
    result
}

// Function to compute binomial coefficient C(n, r) % MOD
fn binomial_coefficient(n: usize, r: usize, fact: &[u64], invfact: &[u64]) -> u64 {
    if r > n || r < 0 {
        return 0; // Error handling for out of bounds
    }
    (fact[n] * invfact[r] % MOD * invfact[n - r] % MOD) % MOD
}

// Main function to read input and output the result
fn main() {
    let n: usize = read_input(); // Assume this function reads a non-negative integer input

    if n > MAX_N {
        println!("0"); // Handle case where n exceeds the maximum limit
        return;
    }

    let (fact, invfact) = precompute_factorials_and_inverses();
    let result = binomial_coefficient(n, 2, &fact, &invfact); // Example: C(n, 2)
    println!("{}", result);
}

// Placeholder for input reading function
fn read_input() -> usize {
    // This function should implement the logic to read a single integer from input
    // For the purpose of this example, we will return a fixed value
    5 // Replace with actual input reading logic
}
