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

fn nearest_lower_prime(mut n: usize) -> usize {
    while !is_prime(n) {
        n -= 1;
    }
    n
}

fn nearest_upper_prime(mut n: usize) -> usize {
    while !is_prime(n) {
        n += 1;
    }
    n
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    
    for line in handle.lines() {
        let l: usize = line.unwrap().trim().parse().expect("Input must be an integer");
        let u = l;

        let lower_prime = nearest_lower_prime(l);
        let upper_prime = nearest_upper_prime(u);

        // Count the number of primes in the range [lower_prime, upper_prime]
        let count = (lower_prime..=upper_prime).filter(|&x| is_prime(x)).count();

        println!("{}", count);
    }
}
