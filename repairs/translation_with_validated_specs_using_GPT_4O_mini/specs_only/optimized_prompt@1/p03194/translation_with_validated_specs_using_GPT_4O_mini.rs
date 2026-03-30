fn largest_prime_product(n: usize, p: usize) -> usize {
    // Preconditions check
    if n == 0 {
        panic!("n must be a positive integer");
    }
    if p == 0 {
        return 0; // If p is zero, the output is zero
    }

    let mut product = 1; // This will hold the final product of prime factors
    let mut factor_count = 0; // To count the total multiplicity of prime factors
    let mut current = p; // A mutable copy of p for factorization

    // Factorization loop
    let mut factor = 2; // Start with the smallest prime
    let mut factors = Vec::new(); // To store the prime factors and their counts

    while factor * factor <= current {
        let mut count = 0; // Count multiplicity of the current factor
        while current % factor == 0 {
            current /= factor;
            count += 1;
        }
        if count > 0 {
            factors.push((factor, count)); // Store the factor and its count
            factor_count += count; // Update total multiplicity
        }
        factor += 1; // Move to the next potential prime factor
    }

    // If there's any prime factor left greater than sqrt(p), it can only be `current`
    if current > 1 {
        factors.push((current, 1)); // This is a prime factor
        factor_count += 1; // Update total multiplicity
    }

    // Calculate the product based on the multiplicity and limit by n
    let mut total_factors_used = 0; // To track how many factors we've used
    for (factor, count) in factors {
        let use_count = count.min(n - total_factors_used); // Use up to n factors
        for _ in 0..use_count {
            product *= factor; // Multiply the product by the factor
        }
        total_factors_used += use_count; // Update the count of factors used
        if total_factors_used >= n {
            break; // Stop if we've used enough factors
        }
    }

    product // Return the computed product
}

fn main() {
    // Example usage
    let n = 3; // Number of factors
    let p = 216; // Integer to be factorized
    let result = largest_prime_product(n, p);
    println!("{}", result); // Should print 6 for the input (3, 216)
}
