fn main() {
    // Read two integers from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Unpack the inputs
    let n = nums[0];
    let m = nums[1];

    // Compute the sum based on the relationship between n and m
    let result = if m == 1 {
        0 // If m is 1, the output is defined to be 0
    } else if m > 1 {
        n + (m - 1) // For m > 1, compute the sum as n + (m - 1)
    } else {
        // If m is less than or equal to 0, behavior is not defined
        // We will return 0 as a safe fallback, but this is not specified
        0
    };

    // Print the result
    println!("{}", result);
}
