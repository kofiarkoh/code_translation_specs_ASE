use std::io::{self, BufRead};
use std::collections::HashSet;

/// Generates a list of prime numbers up to a specified maximum using the Sieve of Eratosthenes.
fn generate_primes(max: usize) -> Vec<usize> {
    let mut is_prime = vec![true; max + 1];
    is_prime[0] = false; // 0 is not prime
    is_prime[1] = false; // 1 is not prime

    for i in 2..=((max as f64).sqrt() as usize) {
        if is_prime[i] {
            for multiple in (i * i..=max).step_by(i) {
                is_prime[multiple] = false;
            }
        }
    }

    is_prime.iter()
        .enumerate()
        .filter_map(|(num, &prime)| if prime { Some(num) } else { None })
        .collect()
}

/// Counts distinct pairs of prime numbers that sum to the target value.
fn count_prime_pairs(primes: &[usize], target: usize) -> usize {
    let mut count = 0;
    let mut seen = HashSet::new();

    for &p in primes {
        if p > target {
            break; // No need to check further if p exceeds target
        }
        let complement = target - p;
        if seen.contains(&complement) {
            count += 1; // Found a valid pair
        }
        seen.insert(p); // Add current prime to seen set
    }

    count
}

fn main() {
    let max_limit = 100; // Define the maximum limit for prime generation
    let primes = generate_primes(max_limit); // Generate primes up to the maximum limit

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        match line {
            Ok(input) => {
                if let Ok(value) = input.trim().parse::<usize>() {
                    if value == 0 {
                        break; // Terminate on zero input
                    }
                    let count = count_prime_pairs(&primes, value);
                    println!("{}", count); // Output the count of prime pairs
                }
                // If input is not a valid integer, no output is produced as per specification
            }
            Err(_) => {
                // Handle potential I/O errors gracefully
                continue; // Ignore and continue on error
            }
        }
    }
}
