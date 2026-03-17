fn main() {
    // Read the number of iterations from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Input must be a positive integer");

    // Ensure N is positive
    if n == 0 {
        return; // Undefined behavior for N <= 0, as per specification
    }

    let mut ans: i64 = 0; // Initialize ans as a long long integer (i64 in Rust)

    // Read N-1 pairs of integers (u, w)
    for _ in 0..(n - 1) {
        let mut pair_input = String::new();
        std::io::stdin().read_line(&mut pair_input).expect("Failed to read line");
        let pair: Vec<i64> = pair_input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Input must be integers"))
            .collect();

        // Ensure we have exactly two integers in the pair
        if pair.len() != 2 {
            return; // Undefined behavior for incorrect input format
        }

        let u = pair[0];
        let w = pair[1];

        // Perform arithmetic operations to update ans
        // The specific operations are not defined in the specification,
        // so we will assume a simple addition for demonstration purposes.
        ans += u + w; // Adjust ans based on the pairs
    }

    // Output the final result
    println!("{}", ans); // Flushes output to ensure visibility
}
