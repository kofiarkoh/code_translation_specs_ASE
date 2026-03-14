use std::io::{self, BufRead};

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn sum_of_first_d_primes(d: usize) -> u64 {
    let mut sum = 0;
    let mut count = 0;
    let mut num = 2; // Start checking for primes from the first prime number

    while count < d {
        if is_prime(num) {
            sum += num;
            count += 1;
        }
        num += 1; // Move to the next number
    }

    sum
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let input = line.unwrap().trim().parse::<usize>();
        
        match input {
            Ok(d) if d > 0 => {
                let result = sum_of_first_d_primes(d);
                println!("{}", result);
            }
            Ok(0) => break, // Terminate on zero input
            _ => continue, // Ignore invalid inputs (as per specification)
        }
    }
}
