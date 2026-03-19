use std::io;

fn main() {
    // Read input values from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input into two integers, x and y
    let mut iter = input.split_whitespace();
    let x: i32 = iter.next().expect("Expected x").parse().expect("Invalid integer for x");
    let y: i32 = iter.next().expect("Expected y").parse().expect("Invalid integer for y");

    // Compute the result based on the relationship between x and y
    let result = compute_result(x, y);
    
    // Print the result as a long long integer
    println!("{}", result);
}

// Function to compute the result based on the specified conditional rules
fn compute_result(x: i32, y: i32) -> i64 {
    // Placeholder for the conditional logic based on x and y
    // The actual conditions should be defined according to the specification
    if x > y {
        (x + y) as i64 // Example condition: if x is greater than y, return their sum
    } else if x < y {
        (y - x) as i64 // Corrected condition: return the positive difference
    } else {
        (x * y) as i64 // Example condition: if x equals y, return their product
    }
}
