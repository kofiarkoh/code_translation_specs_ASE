use std::io::{self, Write};

fn main() {
    // Read input values for n and k
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    // Parse n and k as i64 (equivalent to long long int in C)
    let n: i64 = iter.next().unwrap().parse().expect("Invalid input for n");
    let k: i64 = iter.next().unwrap().parse().expect("Invalid input for k");

    // Initialize the answer counter
    let mut ans = 0;

    // Iterate through the range of values based on n % 693
    for i in 0..693 {
        // Calculate the transformed value of n
        let transformed_n = n - i;

        // Check if the transformed value is non-negative
        if transformed_n >= 0 {
            // Check the modular condition
            if transformed_n % k == 0 {
                ans += 1; // Increment the count if the condition is met
            }
        }
    }

    // Output the result
    println!("{}", ans);
}
