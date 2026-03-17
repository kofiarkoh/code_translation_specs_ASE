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
    (2..=limit).filter(|&n| is_prime(n)).collect()
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut primes = generate_primes(100_000); // Generate primes up to 100,000

    for line in reader.lines() {
        let input = line.unwrap();
        let nums: Vec<usize> = input.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if nums.len() != 3 {
            continue; // Skip invalid input
        }

        let (m, a, b) = (nums[0], nums[1], nums[2]);

        if m == 0 && a == 0 && b == 0 {
            break; // Termination condition
        }

        let mut found_pair = false;
        let mut best_pair = (0, 0);
        
        for &p in &primes {
            for &q in &primes {
                if p * q > m {
                    break; // No need to check further if product exceeds m
                }
                if a * q <= p * b {
                    found_pair = true;
                    if p * q > best_pair.0 * best_pair.1 {
                        best_pair = (p, q); // Update best pair if current is better
                    }
                }
            }
        }

        if found_pair {
            println!("{} {}", best_pair.0, best_pair.1);
        } else {
            println!("0 0"); // No valid pairs found
        }
    }
}
