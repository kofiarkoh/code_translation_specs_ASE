use std::io;

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into two parts and parse them as i64 (equivalent to long long int)
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let n: i64 = parts[0].parse().expect("Invalid input for n");
    let m: i64 = parts[1].parse().expect("Invalid input for m");

    // Initialize the sum variable
    let sum: i64;

    // Compute the sum based on the relationship between n and m
    if m == 1 {
        sum = 0; // If m is 1, the output will be 0
    } else if m > 1 {
        // If m is greater than 1, compute the sum as n + (m - 1)
        sum = n + (m - 1);
    } else {
        // If m is less than or equal to 0, behavior is not defined
        // We will not handle this case explicitly as per the specification
        return; // Exit early, as we cannot define behavior for m <= 0
    }

    // Print the computed sum
    println!("{}", sum);
}
