use std::io::{self, BufRead};

fn main() {
    // Read the number of queries Q
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).expect("Failed to read line");
    
    // Parse Q, ensuring it's a positive integer
    let q: usize = input.trim().parse().expect("Q must be a positive integer");

    // Read the array of integers N
    let mut n_values = Vec::with_capacity(q);
    for _ in 0..q {
        input.clear();
        stdin.lock().read_line(&mut input).expect("Failed to read line");
        let n: usize = input.trim().parse().expect("Each N[i] must be a non-negative integer");
        n_values.push(n);
    }

    // Compute counts based on the multiplicative condition
    let max_n = *n_values.iter().max().unwrap_or(&0);
    let counts = compute_counts(max_n);

    // Output the results for each N[i]
    for &n in &n_values {
        println!("{}", counts[n]);
    }
}

// Function to compute counts based on prime factorization and multiplicative conditions
fn compute_counts(max_n: usize) -> Vec<usize> {
    let mut counts = vec![0; max_n + 1];

    // Count integers satisfying the multiplicative condition
    for i in 2..=max_n {
        // Check if i is prime
        if is_prime(i) {
            // Increment count for prime numbers
            counts[i] = counts[i - 1] + 1;
        } else {
            // For non-prime numbers, carry forward the previous count
            counts[i] = counts[i - 1];
        }
    }

    counts
}

// Function to determine if a number is prime
fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
