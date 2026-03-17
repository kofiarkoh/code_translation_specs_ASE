use std::io;

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as an integer
    let n: usize = input.trim().parse().expect("Input must be a valid integer");

    // Ensure the input meets the precondition
    if n < 2 {
        // Undefined behavior for n < 2 as per specification
        return;
    }

    // Create a vector to count prime factors, initialized to zero
    let mut prime_factors_count = vec![0; n + 1];

    // Perform the Sieve of Eratosthenes to count prime factors
    for i in 2..=n {
        if prime_factors_count[i] == 0 { // i is a prime number
            for multiple in (i..=n).step_by(i) {
                prime_factors_count[multiple] += 1; // Count the prime factor
            }
        }
    }

    // Calculate the final result based on the prime factor counts
    let result: usize = prime_factors_count.iter().sum();

    // Output the computed result
    println!("{}", result);
}
