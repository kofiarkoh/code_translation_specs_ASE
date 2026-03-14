use std::io::{self, BufRead};

const MAX: usize = 10_001;

fn set_prime(max: usize) -> Vec<bool> {
    let mut prime = vec![false; max + 1];

    // Mark even numbers greater than 2 as non-prime
    for i in (4..=max).step_by(2) {
        prime[i] = true;
    }

    // Sieve of Eratosthenes
    for i in (3..=((max as f64).sqrt() as usize)).step_by(2) {
        if !prime[i] {
            for j in (i * 2..=max).step_by(i) {
                prime[j] = true;
            }
        }
    }

    prime
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut prime = set_prime(MAX);

    for line in reader.lines() {
        let n: usize = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break; // Termination condition
        }

        // Search for twin primes
        for i in (7..=n).rev() {
            if !prime[i] && !prime[i - 2] {
                println!("{} {}", i - 2, i);
                break; // Output the first found twin prime pair
            }
        }
    }
}
