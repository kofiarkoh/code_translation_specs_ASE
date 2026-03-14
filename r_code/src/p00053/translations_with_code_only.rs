use std::io::{self, BufRead};
use std::f64;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut s = vec![2, 3]; // Store the first two primes
    let mut n: usize;
    let mut sum: i64;

    loop {
        let mut input = String::new();
        handle.read_line(&mut input).unwrap();
        let d: i32 = input.trim().parse().unwrap();

        if d == 0 {
            break;
        }

        n = 0; // Reset the count of primes
        sum = 0;

        if d <= 2 {
            for i in 0..d {
                sum += s[i as usize] as i64;
            }
        } else {
            sum = 5; // Starting sum with the first two primes
            n = 1; // We already have 2 and 3

            let mut i = 5; // Start checking for primes from 5
            loop {
                let mut sw = 0; // Switch to indicate if a divisor is found
                let k = (i as f64).sqrt() as i64; // Calculate the square root of i

                for &prime in &s {
                    if prime > k {
                        break; // No need to check further if prime exceeds sqrt(i)
                    }
                    if i % prime == 0 {
                        sw = 1; // Found a divisor
                        break;
                    }
                }

                if sw == 0 {
                    n += 1;
                    if n < 1000 {
                        s.push(i); // Store the found prime
                    }
                    sum += i as i64; // Add the prime to the sum
                    if n == d - 1 {
                        break; // Stop if we have found enough primes
                    }
                }
                i += 2; // Check only odd numbers
            }
        }
        println!("{}", sum); // Output the sum
    }
}
