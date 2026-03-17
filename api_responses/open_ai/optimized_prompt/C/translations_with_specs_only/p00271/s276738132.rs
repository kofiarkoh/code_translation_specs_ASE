fn main() {
    // We will read 14 integers from standard input.
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    // Split the input string into parts and parse them as integers.
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid integer input"))
        .collect();

    // Ensure we have exactly 14 integers as specified.
    assert_eq!(numbers.len(), 14, "Expected exactly 14 integers");

    // Compute the differences for each pair and print the results.
    for i in (0..14).step_by(2) {
        let difference = numbers[i] - numbers[i + 1]; // Subtract the second integer from the first
        println!("{}", difference); // Output the result
    }
}
