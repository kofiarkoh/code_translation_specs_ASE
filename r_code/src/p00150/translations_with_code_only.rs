use std::io::{self, BufRead};

const MAX: usize = 10001;

fn set_prime(max: usize) -> Vec<u8> {
    let mut prime = vec![0; max + 1];

    // Mark even numbers greater than 2 as non-prime
    for i in (4..=max).step_by(2) {
        prime[i] = 1;
    }

    // Sieve of Eratosthenes for odd numbers
    for i in (3..=((max as f64).sqrt() as usize)).step_by(2) {
        if prime[i] == 0 {
            for j in (i * 2..=max).step_by(i) {
                prime[j] = 1;
            }
        }
    }

    prime
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let prime = set_prime(MAX);

    for line in handle.lines() {
        let n: i32 = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break;
        }

        for i in (7..=n).rev() {
            if prime[i as usize] + prime[(i - 2) as usize] == 0 {
                println!("{} {}", i - 2, i);
                break;
            }
        }
    }
}
