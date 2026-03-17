fn main() {
    use std::io::{self, BufRead};

    // Read the first line for the number of binary strings to process
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of binary strings
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Expected a non-negative integer");

    // Ensure n is non-negative
    if n == 0 {
        println!("0");
        return;
    }

    // Read the first binary string
    let first_binary_string = lines.next().unwrap().unwrap();
    let mut result = process_binary_string(&first_binary_string);

    // Process subsequent binary strings
    for _ in 1..n {
        let binary_string = lines.next().unwrap().unwrap();
        result = (result + process_binary_string(&binary_string)) % 998244353;
    }

    // Print the final result
    println!("{}", result);
}

// Function to process a binary string and return its integer value
fn process_binary_string(binary_string: &str) -> usize {
    // Check if the binary string is valid (only contains '0' and '1')
    if !binary_string.chars().all(|c| c == '0' || c == '1') {
        panic!("Invalid binary string: {}", binary_string);
    }

    // Convert the binary string to an integer
    usize::from_str_radix(binary_string, 2).expect("Failed to convert binary string to integer")
}
