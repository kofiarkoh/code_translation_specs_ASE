fn prime_factor_count(n: usize) -> Vec<usize> {
    // Create a vector to store counts of prime factors for each integer
    let mut counts = vec![0; n + 1];

    // Count prime factors for each integer from 2 to n
    for i in 2..=n {
        let mut num = i;
        let mut factor = 2;

        // Factorize the number
        while num > 1 {
            while num % factor == 0 {
                counts[factor] += 1; // Increment count for this prime factor
                num /= factor; // Divide num by the prime factor
            }
            factor += 1; // Move to the next potential prime factor
        }
    }

    counts
}

fn calculate_result(counts: &[usize]) -> usize {
    // Calculate the result based on the counts of prime factors
    let mut result = 1; // Start with 1 to multiply counts

    for &count in counts.iter().skip(2) { // Skip index 0 and 1 as they are not prime
        if count > 0 {
            result *= count + 1; // Use (count + 1) for the formula
        }
    }

    result // Return the computed result
}

fn main() {
    let n: usize = 100; // Example input, replace with actual input as needed
    assert!(n >= 2); // Ensure precondition is met

    let counts = prime_factor_count(n); // Get prime factor counts
    let result = calculate_result(&counts); // Calculate the final result

    println!("{}", result); // Output the result
}
