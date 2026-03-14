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

fn find_twin_primes(limit: usize) {
    let mut primes = Vec::new();
    
    // Collect all primes up to the limit
    for num in 2..=limit {
        if is_prime(num) {
            primes.push(num);
        }
    }

    // Check for twin primes
    for window in primes.windows(2) {
        if window[1] - window[0] == 2 {
            println!("{} {}", window[0], window[1]);
            break; // Only output the first found pair
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let input = line.unwrap().trim().parse::<usize>().unwrap();
        
        if input == 0 {
            break; // Termination condition
        }
        
        if input >= 7 && input <= 10_000 {
            find_twin_primes(input);
        }
    }
}
