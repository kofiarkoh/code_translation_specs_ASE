use std::io::{self, BufRead};

fn main() {
    // Define the maximum limit for prime number generation
    const MAX_LIMIT: usize = 100; // Adjust this limit as needed

    // Generate the prime table
    let prime_table = generate_prime_table(MAX_LIMIT);

    // Read input values until a zero is encountered
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = line.expect("Failed to read line");
        let target_sum: usize = input.trim().parse().expect("Invalid input");

        // Terminate on zero input
        if target_sum == 0 {
            break;
        }

        // Count and output the distinct pairs of primes that sum to the target value
        let count = count_prime_pairs(&prime_table, target_sum);
        println!("{}", count);
    }
}

// Function to generate a table of prime numbers up to a specified limit
fn generate_prime_table(limit: usize) -> Vec<u8> {
    let mut is_prime = vec![1; limit + 1]; // 1 for non-prime, 0 for prime
    is_prime[0] = 1; // 0 is not prime
    is_prime[1] = 1; // 1 is not prime

    for num in 2..=limit {
        if is_prime[num] == 0 { // If num is prime
            for multiple in (num * 2..=limit).step_by(num) {
                is_prime[multiple] = 1; // Mark multiples as non-prime
            }
        }
    }
    is_prime
}

// Function to count distinct pairs of prime numbers that sum to a given value
fn count_prime_pairs(prime_table: &[u8], target_sum: usize) -> usize {
    let mut count = 0;
    let mut seen = std::collections::HashSet::new();

    for i in 2..=target_sum / 2 {
        if prime_table[i] == 0 && prime_table[target_sum - i] == 0 {
            // Check if the pair (i, target_sum - i) is distinct
            let pair = if i < target_sum - i {
                (i, target_sum - i)
            } else {
                (target_sum - i, i)
            };
            if seen.insert(pair) {
                count += 1; // Count the distinct pair
            }
        }
    }
    count
}
