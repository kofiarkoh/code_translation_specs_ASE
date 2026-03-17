fn main() {
    // Read inputs from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let inputs: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Unpack inputs
    let n = inputs[0]; // N as long long (i64 in Rust)
    let mut a = inputs[1] as i32; // A as int (i32 in Rust)
    let mut b = inputs[2] as i32; // B as int (i32 in Rust)

    // Calculate effective iterations (N % 12)
    let iterations = (n % 12).max(0); // Ensure non-negative iterations

    // Perform arithmetic operations for the number of iterations
    for _ in 0..iterations {
        a += 1; // Example operation: increment A
        b += 2; // Example operation: increment B
    }

    // Output the final values of A and B
    println!("{} {}", a, b);

    // Return 0 upon successful execution
    std::process::exit(0);
}
