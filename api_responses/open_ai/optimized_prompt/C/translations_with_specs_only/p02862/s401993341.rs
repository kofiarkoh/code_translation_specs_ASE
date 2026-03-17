// Define a constant for the modulus to ensure calculations remain within bounds
const MOD: u64 = 1_000_000_007;

// Function to calculate factorial modulo MOD
fn factorial(n: usize) -> u64 {
    let mut result = 1;
    for i in 1..=n {
        result = (result * i as u64) % MOD;
    }
    result
}

// Function to calculate modular inverse using Fermat's Little Theorem
fn mod_inverse(a: u64) -> u64 {
    let mut m = MOD - 2; // MOD is prime, so we can use Fermat's theorem
    let mut result = 1;
    let mut base = a;

    while m > 0 {
        if m % 2 == 1 {
            result = (result * base) % MOD;
        }
        base = (base * base) % MOD;
        m /= 2;
    }
    result
}

// Function to calculate the number of distinct paths
fn distinct_paths(x: i32, y: i32) -> u64 {
    // Check if the sum of coordinates is divisible by 3
    if (x + y) % 3 != 0 || x < 0 || y < 0 {
        return 0; // Return 0 if conditions are not met
    }

    // Calculate the number of steps
    let steps = (x + y) as usize;
    let half_steps = (steps / 2) as usize;

    // Precompute factorials and inverse factorials
    let mut fact = vec![0; steps + 1];
    let mut inv_fact = vec![0; steps + 1];
    
    for i in 0..=steps {
        fact[i] = factorial(i);
    }
    
    for i in 0..=steps {
        inv_fact[i] = mod_inverse(fact[i]);
    }

    // Calculate the number of distinct paths using combinations
    let paths = (fact[steps] * inv_fact[half_steps] % MOD) * inv_fact[steps - half_steps] % MOD;
    paths
}

// Main function to demonstrate the functionality
fn main() {
    let x = 3; // Example input for x coordinate
    let y = 3; // Example input for y coordinate
    let result = distinct_paths(x, y);
    println!("{}", result); // Output the result
}
