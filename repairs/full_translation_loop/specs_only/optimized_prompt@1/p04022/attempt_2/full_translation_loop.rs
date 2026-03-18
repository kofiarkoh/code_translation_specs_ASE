use std::io::{self, BufRead};

fn main() {
    // Read the number of elements
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // If no integers are processed, output will be zero
    if n == 0 {
        println!("0");
        return;
    }

    // Read the sequence of integers
    let mut numbers: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let number: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        numbers.push(number);
    }

    // Count of specific conditions based on prime factorization
    let mut count: i64 = 0;

    // Function to count distinct prime factors of a number
    fn count_distinct_prime_factors(num: i64) -> i64 {
        let mut count = 0;
        let mut n = num;

        // Check for number of 2s that divide n
        if n % 2 == 0 {
            count += 1;
            while n % 2 == 0 {
                n /= 2;
            }
        }

        // Check for odd factors from 3 to sqrt(n)
        let mut factor = 3;
        while factor * factor <= n {
            if n % factor == 0 {
                count += 1;
                while n % factor == 0 {
                    n /= factor;
                }
            }
            factor += 2;
        }

        // If n is a prime number greater than 2
        if n > 2 {
            count += 1;
        }

        count
    }

    // Process each number and count based on distinct prime factors
    for &number in &numbers {
        if number > 1 {
            let distinct_prime_count = count_distinct_prime_factors(number);
            if distinct_prime_count > 0 {
                count += 1; // Increment count for each number with distinct prime factors
            }
        }
    }

    // Output the final count
    println!("{}", count);
}
