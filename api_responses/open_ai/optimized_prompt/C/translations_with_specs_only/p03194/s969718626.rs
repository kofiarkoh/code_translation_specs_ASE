fn main() {
    // Example inputs
    let number: i64 = 28; // Non-negative integer input
    let threshold: i64 = 2; // Positive integer input

    // Ensure preconditions are met
    if number < 0 || threshold <= 0 {
        // Behavior is not defined if preconditions are not met
        return;
    }

    let product = calculate_largest_prime_product(number, threshold);
    println!("{}", product);
}

fn calculate_largest_prime_product(number: i64, threshold: i64) -> i64 {
    let prime_factors = prime_factorization(number);
    let factor_count = prime_factors.len() as i64;

    // Calculate the exponent for each prime factor
    let exponent = factor_count / threshold;

    // Calculate the product of prime factors raised to the calculated exponent
    let mut result = 1;
    for &factor in &prime_factors {
        // Use `checked_mul` to avoid overflow; if overflow occurs, return the maximum i64
        result = match result.checked_mul(factor.pow(exponent as u32)) {
            Some(value) => value,
            None => i64::MAX, // Return maximum i64 if overflow occurs
        };
    }

    // Ensure the result is non-negative and respects the invariant
    if threshold > 1 && result > number {
        return number; // Output is guaranteed to be less than or equal to the first input
    }

    result
}

fn prime_factorization(mut n: i64) -> Vec<i64> {
    let mut factors = Vec::new();
    let mut divisor = 2;

    while n > 1 {
        while n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        }
        divisor += 1;
    }

    factors
}
