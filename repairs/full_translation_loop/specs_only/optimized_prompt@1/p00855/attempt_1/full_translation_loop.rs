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

fn largest_prime_leq(n: usize) -> usize {
    let mut candidate = n;
    while candidate > 1 {
        if is_prime(candidate) {
            return candidate;
        }
        candidate -= 1;
    }
    0 // Return 0 if no prime is found (though this should not happen for n >= 2)
}

fn smallest_prime_geq(n: usize) -> usize {
    let mut candidate = n;
    while !is_prime(candidate) {
        candidate += 1;
    }
    candidate
}

fn count_primes_in_range(lower: usize, upper: usize) -> usize {
    (lower..=upper).filter(|&n| is_prime(n)).count()
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    
    for line in handle.lines() {
        if let Ok(input) = line {
            if let Ok(upper) = input.trim().parse::<usize>() {
                let lower = largest_prime_leq(upper);
                let upper = smallest_prime_geq(upper);
                let count = count_primes_in_range(lower, upper);
                println!("{}", count);
            }
            // Invalid input handling is not defined as per the specification
        }
    }
}
