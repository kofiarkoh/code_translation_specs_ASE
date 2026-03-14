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

fn generate_primes(limit: usize) -> Vec<usize> {
    (0..=limit).filter(|&n| is_prime(n)).collect()
}

fn count_prime_pairs(sum: usize, primes: &[usize]) -> usize {
    let mut count = 0;
    let mut seen = std::collections::HashSet::new();

    for &p in primes.iter() {
        if p > sum / 2 {
            break; // No need to check beyond half of the sum
        }
        let complement = sum - p;
        if complement >= p && seen.contains(&complement) {
            count += 1;
        }
        seen.insert(p);
    }
    count
}

fn main() {
    let limit = 5000;
    let primes = generate_primes(limit);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(input) = line {
            let num: usize = input.trim().parse().unwrap(); // Assumes valid input as per specification
            if num == 0 {
                break; // Terminate on zero input
            }
            if num > 0 {
                let count = count_prime_pairs(num, &primes);
                println!("{}", count);
            }
        }
    }
}
