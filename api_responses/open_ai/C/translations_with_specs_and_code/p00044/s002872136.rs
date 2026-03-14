use std::io::{self, BufRead};

fn generate_primes(limit: usize) -> Vec<usize> {
    let mut primes = vec![2];
    for a in (3..limit).step_by(2) {
        let mut is_prime = true;
        for &p in &primes {
            if p * p > a {
                break;
            }
            if a % p == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(a);
        }
    }
    primes.push(limit); // Adding the limit as a sentinel value
    primes
}

fn main() {
    let limit = 60000;
    let primes = generate_primes(limit);
    
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                if let Ok(a) = input.trim().parse::<usize>() {
                    if a < limit {
                        let mut n = 0;
                        while n < primes.len() && a >= primes[n] {
                            n += 1;
                        }
                        if n > 0 && primes[n - 1] == a {
                            // Exact match
                            println!("{} {}", primes[n - 2], primes[n]);
                        } else {
                            // Nearest primes
                            println!("{} {}", primes[n - 1], primes[n]);
                        }
                    }
                }
            }
            Err(_) => break, // Terminate on unexpected input
        }
    }
}
