fn main() {
    // Read input values for time intervals and coefficients
    let input = std::io::stdin();
    let mut buffer = String::new();
    input.read_line(&mut buffer).expect("Failed to read line");
    
    // Parse the input into six long long integers
    let values: Vec<i64> = buffer
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Ensure we have exactly six inputs
    if values.len() != 6 {
        panic!("Expected exactly six long long integers as input");
    }

    let t1 = values[0];
    let t2 = values[1];
    let a1 = values[2];
    let a2 = values[3];
    let b1 = values[4];
    let b2 = values[5];

    // Calculate the linear combinations
    let lhs = a1 * t1 + a2 * t2;
    let rhs = b1 * t1 + b2 * t2;

    // Check for equality and compute the result
    if lhs == rhs {
        // If the linear combinations are equal, output "infinity"
        println!("infinity");
    } else {
        // Otherwise, compute the difference and output it
        let result = lhs - rhs;
        println!("{}", result);
    }
}
