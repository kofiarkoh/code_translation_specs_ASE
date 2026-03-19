use std::io::{self, BufRead};

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

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let l: usize = match line {
            Ok(input) => input.trim().parse().unwrap(), // Assumes valid input as per spec
            Err(_) => continue, // If reading fails, skip to the next line
        };

        // Find the largest prime less than or equal to l
        let mut lower = l;
        while lower > 0 && !is_prime(lower) {
            lower -= 1; // Decrement until a prime is found
        }

        // Find the smallest prime greater than or equal to l
        let mut upper = l;
        while !is_prime(upper) {
            upper += 1; // Increment until a prime is found
        }

        // Count primes in the range [lower, upper]
        let prime_count = (lower..=upper).filter(|&n| is_prime(n)).count();
        println!("{}", prime_count); // Output the count of primes in the range
    }
}
