fn main() {
    // Initialize the list of prime numbers
    let primes = generate_primes(100_000);
    
    // Read inputs in a loop until all three are zero
    loop {
        let (m, a, b) = read_inputs();
        if m == 0 && a == 0 && b == 0 {
            break; // Exit the loop if all inputs are zero
        }
        
        // Find the optimal pair of primes
        let (p, q) = find_optimal_primes(&primes, m, a, b);
        
        // Print the result
        println!("{} {}", p, q);
    }
}

// Function to generate all prime numbers up to a given limit using the Sieve of Eratosthenes
fn generate_primes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false; // 0 is not prime
    is_prime[1] = false; // 1 is not prime

    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i..=limit).step_by(i) {
                is_prime[j] = false; // Mark multiples of i as non-prime
            }
        }
    }

    // Collect all prime numbers into a vector
    is_prime.iter()
        .enumerate()
        .filter_map(|(num, &is_prime)| if is_prime { Some(num) } else { None })
        .collect()
}

// Function to read three integer inputs from the user
fn read_inputs() -> (usize, usize, usize) {
    use std::io::{self, Write};
    
    let mut input = String::new();
    print!("Enter m, a, b: ");
    io::stdout().flush().unwrap(); // Ensure prompt is displayed
    io::stdin().read_line(&mut input).unwrap();
    
    let nums: Vec<usize> = input.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    // Return the first three numbers or zeroes if not enough inputs
    let m = *nums.get(0).unwrap_or(&0);
    let a = *nums.get(1).unwrap_or(&0);
    let b = *nums.get(2).unwrap_or(&0);
    
    (m, a, b)
}

// Function to find the optimal pair of primes (p, q) based on the constraints
fn find_optimal_primes(primes: &[usize], m: usize, a: usize, b: usize) -> (usize, usize) {
    let mut best_pair = (0, 0);
    let mut max_product = 0;

    for &p in primes.iter().filter(|&&p| p <= b) {
        for &q in primes.iter().filter(|&&q| q >= a) {
            let product = p * q;
            if product <= m && product > max_product {
                max_product = product;
                best_pair = (p, q);
            }
        }
    }

    best_pair // Return the best pair found, or (0, 0) if none found
}
