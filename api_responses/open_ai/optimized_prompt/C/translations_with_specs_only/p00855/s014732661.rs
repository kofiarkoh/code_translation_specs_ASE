use std::io::{self, BufRead};

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as i32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn nearest_lower_prime(mut n: i32) -> i32 {
    while n >= 2 {
        if is_prime(n) {
            return n;
        }
        n -= 1;
    }
    2 // The smallest prime number
}

fn nearest_upper_prime(mut n: i32) -> i32 {
    while is_prime(n) == false {
        n += 1;
    }
    n
}

fn count_primes_in_range(lower: i32, upper: i32) -> i32 {
    (lower..=upper).filter(|&n| is_prime(n)).count() as i32
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        if let Ok(input) = line {
            if let Ok(lower_bound) = input.trim().parse::<i32>() {
                let lower_prime = nearest_lower_prime(lower_bound);
                let upper_prime = nearest_upper_prime(lower_bound);
                let prime_count = count_primes_in_range(lower_prime, upper_prime);
                println!("{}", prime_count);
            }
        }
    }
}
