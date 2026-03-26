use std::io::{self, BufRead};

const MAX: usize = 50000;

fn make_prime_table(table: &mut [u8; MAX]) -> i32 {
    // Initialize the table with 1 (non-prime)
    for i in 0..MAX {
        table[i] = 1; // Assume all are non-prime initially
    }
    table[0] = 0; // 0 is not prime
    table[1] = 0; // 1 is not prime

    // Sieve of Eratosthenes to find all primes up to MAX
    for i in 2..(MAX as f64).sqrt() as usize + 1 {
        if table[i] == 1 { // If i is prime
            for j in (i * i..MAX).step_by(i) {
                table[j] = 0; // Mark multiples of i as non-prime
            }
        }
    }
    0 // Return 0 upon successful execution
}

fn count_prime_pairs(n: usize, table: &[u8; MAX]) -> usize {
    let mut count = 0;
    for i in 2..=n / 2 {
        if table[i] == 1 && table[n - i] == 1 {
            count += 1; // Found a pair (i, n - i)
        }
    }
    count
}

fn main() {
    let mut prime_table = [0; MAX]; // Array to hold prime status
    make_prime_table(&mut prime_table); // Populate the prime table

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = line.unwrap().trim().parse::<usize>().unwrap();
        if input == 0 {
            break; // Terminate on input of 0
        }
        let count = count_prime_pairs(input, &prime_table);
        println!("{}", count); // Output the count of prime pairs
    }
}
