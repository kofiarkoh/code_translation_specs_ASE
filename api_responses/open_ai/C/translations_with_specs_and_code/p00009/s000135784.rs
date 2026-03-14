use std::io::{self, BufRead};

const MAX: usize = 1_000_000;

fn prime_search_eratos() -> Vec<bool> {
    let mut prime = vec![true; MAX + 1];
    prime[0] = false; // 0 is not prime
    prime[1] = false; // 1 is not prime

    let limit = (MAX as f64).sqrt() as usize;
    for i in 2..=limit {
        if !prime[i] {
            continue;
        }
        for j in (i * 2..=MAX).step_by(i) {
            prime[j] = false;
        }
    }
    prime
}

fn main() {
    let prime = prime_search_eratos();
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        if let Ok(input) = line {
            if let Ok(n) = input.trim().parse::<usize>() {
                if n <= MAX {
                    let count = prime.iter().take(n + 1).filter(|&&is_prime| is_prime).count();
                    println!("{}", count);
                }
            }
        }
    }
}
