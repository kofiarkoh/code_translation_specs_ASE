use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of integers to process
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // If n is zero, output 0 and return
    if n == 0 {
        println!("0");
        return;
    }

    // Read the integers into a vector
    let mut a: Vec<u64> = Vec::with_capacity(n);
    for _ in 0..n {
        let value: u64 = lines.next().unwrap().unwrap().parse().unwrap();
        a.push(value);
    }

    // Precompute primes up to a reasonable limit for factorization
    let limit = 100; // Arbitrary limit for prime generation
    let primes = sieve_of_eratosthenes(limit);

    // Count integers that meet the specified conditions
    let count = a.iter().filter(|&&x| meets_conditions(x, &primes)).count();

    // Output the result
    println!("{}", count);
}

// Function to generate a list of primes using the Sieve of Eratosthenes
fn sieve_of_eratosthenes(limit: usize) -> Vec<u64> {
    let mut is_prime = vec![true; limit + 1];
    let mut primes = Vec::new();

    for p in 2..=limit {
        if is_prime[p] {
            primes.push(p as u64);
            for multiple in (p * p..=limit).step_by(p) {
                is_prime[multiple] = false;
            }
        }
    }
    primes
}

// Function to check if a number meets the specified conditions based on its prime factorization
fn meets_conditions(x: u64, primes: &[u64]) -> bool {
    let mut temp = x;
    let mut prime_factors = vec![];

    for &prime in primes {
        if prime * prime > temp {
            break;
        }
        let mut count = 0;
        while temp % prime == 0 {
            temp /= prime;
            count += 1;
        }
        if count > 0 {
            prime_factors.push((prime, count));
        }
    }
    if temp > 1 {
        prime_factors.push((temp, 1)); // Remaining prime factor
    }

    // Check for squares and cubes of primes
    let has_square = prime_factors.iter().any(|&(_, count)| count >= 2);
    let has_cube = prime_factors.iter().any(|&(_, count)| count >= 3);

    // Return true if the number has at least one square or cube
    has_square || has_cube
}
