use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    // Define the maximum limit for prime generation
    const MAX_LIMIT: usize = 100; // Adjust this limit as needed

    // Generate a table of prime numbers up to MAX_LIMIT
    let primes = generate_primes(MAX_LIMIT);

    // Read input values from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    for line in reader.lines() {
        // Handle potential I/O errors
        match line {
            Ok(input) => {
                // Parse the input as an integer
                if let Ok(value) = input.trim().parse::<usize>() {
                    // Terminate on zero input
                    if value == 0 {
                        break;
                    }
                    // Count and output the number of distinct prime pairs
                    let count = count_prime_pairs(&primes, value);
                    println!("{}", count);
                }
            }
            Err(_) => {
                // If there's an error reading the line, we ignore it as per the spec
                continue;
            }
        }
    }
}

// Function to generate a list of prime numbers up to a given limit
fn generate_primes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false; // 0 is not prime
    is_prime[1] = false; // 1 is not prime

    for num in 2..=limit {
        if is_prime[num] {
            // Mark multiples of num as non-prime
            let mut multiple = num * 2;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += num;
            }
        }
    }

    // Collect all prime numbers into a vector
    (2..=limit).filter(|&x| is_prime[x]).collect()
}

// Function to count distinct pairs of prime numbers that sum to a given value
fn count_prime_pairs(primes: &[usize], target_sum: usize) -> usize {
    let mut seen = HashSet::new();
    let mut count = 0;

    for &prime in primes {
        if prime > target_sum {
            break; // No need to check further if prime exceeds target_sum
        }
        let complement = target_sum - prime;
        // Check if the complement is a prime and has not been seen yet
        if complement > prime && primes.contains(&complement) {
            count += 1; // Found a distinct pair
            seen.insert((prime, complement)); // Store the pair to avoid duplicates
        }
    }

    count
}
