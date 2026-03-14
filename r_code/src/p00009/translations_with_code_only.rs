use std::io::{self, BufRead};
use std::f64;

const MAX: usize = 1_000_000;

struct PrimeSieve {
    primes: [u8; MAX],
}

impl PrimeSieve {
    fn new() -> Self {
        let mut primes = [1; MAX];
        primes[0] = 0; // 0 is not prime
        primes[1] = 0; // 1 is not prime

        let limit = (MAX as f64).sqrt() as usize + 1;
        for i in 2..=limit {
            if primes[i] == 0 {
                continue; // Skip non-prime numbers
            }
            for j in (i * 2..MAX).step_by(i) {
                primes[j] = 0; // Mark multiples of i as non-prime
            }
        }

        PrimeSieve { primes }
    }

    fn count_primes_up_to(&self, n: usize) -> usize {
        (0..=n).filter(|&i| self.primes[i] == 1).count()
    }
}

fn main() {
    let prime_sieve = PrimeSieve::new();
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if let Ok(n) = line.trim().parse::<usize>() {
            let count = prime_sieve.count_primes_up_to(n);
            println!("{}", count);
        }
    }
}
