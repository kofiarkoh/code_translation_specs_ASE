use std::io::{self, BufRead};

fn main() {
    // Define the maximum limit for prime number generation
    const MAX_LIMIT: usize = 100_000; // Adjust this limit as needed

    // Create a boolean array to store prime indicators
    let mut is_prime = vec![true; MAX_LIMIT + 1];
    is_prime[0] = false; // 0 is not prime
    is_prime[1] = false; // 1 is not prime

    // Sieve of Eratosthenes to identify prime numbers
    for i in 2..=((MAX_LIMIT as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i)..=MAX_LIMIT {
                is_prime[j] = false; // Mark multiples of i as non-prime
            }
        }
    }

    // Precompute the cumulative count of prime numbers
    let mut prime_count = vec![0; MAX_LIMIT + 1];
    for i in 1..=MAX_LIMIT {
        prime_count[i] = prime_count[i - 1] + if is_prime[i] { 1 } else { 0 };
    }

    // Read input values until EOF
    let stdin = io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        if let Ok(value) = line {
            if let Ok(n) = value.trim().parse::<usize>() {
                if n <= MAX_LIMIT {
                    // Output the count of prime numbers less than or equal to n
                    println!("{}", prime_count[n]);
                }
            }
        }
    }
}
