use std::io::{self, BufRead};

/// Function to check if a number is prime.
fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false; // No primes less than 2
    }
    for i in 2..=((n as f64).sqrt() as i32) {
        if n % i == 0 {
            return false; // Found a divisor, not prime
        }
    }
    true // No divisors found, it's prime
}

/// Function to find the largest prime less than or equal to `l`.
fn largest_prime_leq(l: i32) -> i32 {
    for i in (2..=l).rev() {
        if is_prime(i) {
            return i; // Return the largest prime found
        }
    }
    0 // If no prime is found, return 0 (as per the assumption)
}

/// Function to find the smallest prime greater than or equal to `u`.
fn smallest_prime_geq(u: i32) -> i32 {
    let mut i = u;
    while !is_prime(i) {
        i += 1; // Increment until a prime is found
    }
    i // Return the first prime found
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for `l`
    let l: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    // Read the second line for `s` (not used)
    let _s: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    // Read the third line for `u`
    let u: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    // Find the nearest primes
    let largest_prime = largest_prime_leq(l);
    let smallest_prime = smallest_prime_geq(u);

    // Calculate the difference
    let difference = smallest_prime - largest_prime;

    // Output the result
    println!("{}", difference);
}
