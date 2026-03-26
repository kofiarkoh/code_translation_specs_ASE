use std::io;

fn main() {
    // Read input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as an integer
    let n: usize = input.trim().parse().expect("Input must be a valid integer");

    // Ensure n is at least 2
    if n < 2 {
        return; // Undefined behavior for n < 2, as per specification
    }

    // Initialize the yaku array to hold counts of prime factors
    let mut yaku = [0; 101]; // Fixed size array as per specification

    // Calculate prime factors for each integer from 2 to n
    for i in 2..=n {
        let mut num = i;
        let mut factor = 2;

        // Factorization process
        while factor * factor <= num {
            while num % factor == 0 {
                yaku[factor] += 1; // Count the prime factor
                num /= factor; // Reduce num by the factor
            }
            factor += 1;
        }
        // If num is still greater than 1, it is a prime factor
        if num > 1 {
            yaku[num] += 1; // Count the prime factor
        }
    }

    // Compute the final result based on the yaku array
    let mut result = 1; // Start with 1 for multiplication
    for &count in yaku.iter() {
        if count > 0 {
            result *= count + 1; // Each prime factor contributes (count + 1) to the combinations
        }
    }

    // Print the result
    println!("{}", result);
}
