use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Vector to store the first 1000 prime numbers
    let mut primes = vec![2, 3];
    
    for line in handle.lines() {
        let d: i32 = line.unwrap().trim().parse().unwrap();
        
        // Terminate on zero input
        if d == 0 {
            break;
        }

        let mut sum = 0;

        // If d is 1 or 2, we can directly sum the first d primes
        if d <= 2 {
            for i in 0..d as usize {
                sum += primes[i];
            }
        } else {
            sum = 5; // Starting sum for d > 2
            let mut n = 2; // We already have 2 primes (2 and 3)

            // Start checking for primes from 5 onwards
            let mut candidate = 5;
            while n < d as usize {
                let mut is_prime = true;
                let limit = (candidate as f64).sqrt() as usize;

                // Check divisibility with known primes
                for &prime in &primes {
                    if prime > limit {
                        break;
                    }
                    if candidate % prime == 0 {
                        is_prime = false;
                        break;
                    }
                }

                if is_prime {
                    primes.push(candidate);
                    sum += candidate;
                    n += 1;
                }

                candidate += 2; // Check only odd numbers
            }
        }

        // Output the sum of the first d primes
        println!("{}", sum);
    }
}
