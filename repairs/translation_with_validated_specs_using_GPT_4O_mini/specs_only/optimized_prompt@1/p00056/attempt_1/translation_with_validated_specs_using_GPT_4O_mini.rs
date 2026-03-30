use std::io::{self, Write};

const MAX_VALUE: usize = 100; // Define the maximum value for primes

fn main() {
    let mut input = String::new();
    // Read input from standard input
    if io::stdin().read_line(&mut input).is_err() {
        return; // If reading fails, terminate without output
    }

    // Attempt to parse the input as a non-negative integer
    let n: usize = match input.trim().parse() {
        Ok(num) if num <= MAX_VALUE => num,
        _ => return, // Ignore invalid input or input exceeding the maximum
    };

    if n == 0 {
        return; // Terminate without output if n is 0
    }

    // Generate the prime table
    let primes = generate_prime_table(MAX_VALUE);
    // Count distinct pairs of primes that sum to n
    let count = count_prime_pairs(n, &primes);
    
    // Print the result
    println!("{}", count);
}

// Function to generate a table of primes using the Sieve of Eratosthenes
fn generate_prime_table(max: usize) -> Vec<bool> {
    let mut is_prime = vec![true; max + 1];
    is_prime[0] = false; // 0 is not prime
    is_prime[1] = false; // 1 is not prime

    for i in 2..=((max as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i)..=max {
                is_prime[j] = false; // Mark multiples of i as non-prime
            }
        }
    }
    is_prime
}

// Function to count distinct pairs of primes that sum to n
fn count_prime_pairs(n: usize, primes: &[bool]) -> usize {
    let mut count = 0;
    let mut seen = std::collections::HashSet::new();

    for p1 in 2..=n {
        if primes[p1] {
            let p2 = n - p1;
            if p2 >= p1 && p2 < primes.len() && primes[p2] {
                // Ensure p1 <= p2 and p2 is within bounds
                seen.insert((p1, p2)); // Store the pair in a set to ensure uniqueness
            }
        }
    }
    seen.len() // Return the count of unique pairs
}
