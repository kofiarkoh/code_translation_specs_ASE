use std::io::{self, BufRead};

fn main() {
    // Define the limit for prime number generation
    const LIMIT: usize = 100; // You can adjust this limit as needed
    let primes = generate_primes(LIMIT);

    // Read input until EOF
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                if let Ok(value) = input.trim().parse::<usize>() {
                    if value < LIMIT {
                        let (lower, upper) = find_nearest_primes(value, &primes);
                        println!("{} {}", lower, upper);
                    }
                }
                // If input is not a valid integer or exceeds the limit, we simply skip it
            }
            Err(_) => break, // Terminate on read error
        }
    }
}

// Function to generate a list of prime numbers up to a specified limit
fn generate_primes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false; // 0 is not prime
    is_prime[1] = false; // 1 is not prime

    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for multiple in (i * i..=limit).step_by(i) {
                is_prime[multiple] = false;
            }
        }
    }

    is_prime.iter()
        .enumerate()
        .filter_map(|(num, &is_prime)| if is_prime { Some(num) } else { None })
        .collect()
}

// Function to find the nearest primes surrounding a given input value
fn find_nearest_primes(value: usize, primes: &[usize]) -> (usize, usize) {
    let mut lower = 0;
    let mut upper = 0;

    for &prime in primes {
        if prime <= value {
            lower = prime; // Update lower prime
        }
        if prime > value && upper == 0 {
            upper = prime; // Set upper prime on first encounter
        }
    }

    (lower, upper)
}
```

### Explanation of Choices:
1. **Prime Generation**: The `generate_primes` function uses the Sieve of Eratosthenes to efficiently generate prime numbers up to a specified limit. This ensures that the list of primes is sorted and correctly identified.
  
2. **Input Handling**: The program reads from standard input until EOF. It checks if the input can be parsed as a non-negative integer and ensures it does not exceed the limit.

3. **Output**: For each valid input, the program outputs the nearest primes. If the input is invalid or exceeds the limit, it simply skips outputting anything for that input.

4. **Error Handling**: The program terminates gracefully on read errors and does not produce output for invalid inputs, adhering to the specification's requirements.

5. **Performance**: The program is designed to handle multiple queries efficiently by precomputing the list of primes, which allows for quick lookups.
