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
            Ok(input) => input.trim().parse().expect("Input must be an integer"),
            Err(_) => continue, // If reading fails, skip to the next line
        };

        let mut lower = l;
        while lower > 0 && !is_prime(lower) {
            lower -= 1; // Decrement until a prime is found
        }

        let mut upper = l;
        while !is_prime(upper) {
            upper += 1; // Increment until a prime is found
        }

        // Output the count of primes in the range [lower, upper]
        println!("{}", upper - lower);
    }
}
