use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of integers to process
    let q: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => {
            eprintln!("Error reading input.");
            std::process::exit(1);
        }
    };

    // If Q is 0 or negative, terminate early
    if q == 0 {
        std::process::exit(0);
    }

    // Prepare to store counts
    let mut counts = vec![0; q];

    // Process each integer
    for i in 0..q {
        let n: usize = match lines.next() {
            Some(Ok(line)) => line.trim().parse().unwrap_or(0),
            _ => {
                eprintln!("Error reading input.");
                std::process::exit(1);
            }
        };

        // Calculate the number of unique prime factors
        counts[i] = count_unique_prime_factors(n);
    }

    // Output the counts
    for count in counts {
        println!("{}", count);
    }

    // Return 0 upon successful completion
    std::process::exit(0);
}

// Function to count unique prime factors of a given number
fn count_unique_prime_factors(mut n: usize) -> usize {
    let mut count = 0;
    let mut factor = 2;

    // Check for number of 2s that divide n
    if n % factor == 0 {
        count += 1; // Found a unique prime factor
        while n % factor == 0 {
            n /= factor;
        }
    }

    // Check for odd factors from 3 to sqrt(n)
    factor = 3;
    while factor * factor <= n {
        if n % factor == 0 {
            count += 1; // Found a unique prime factor
            while n % factor == 0 {
                n /= factor;
            }
        }
        factor += 2; // Move to the next odd number
    }

    // If n is still greater than 2, then it is prime
    if n > 2 {
        count += 1; // Found a unique prime factor
    }

    count
}
