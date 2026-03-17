fn main() {
    use std::io::{self, BufRead};

    // Read the number of binary strings to process
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line as the number of binary strings
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Input must be a non-negative integer");

    // Ensure the number of binary strings is non-negative
    if n == 0 {
        println!("0");
        return;
    }

    // Read the initial binary string
    let initial_binary_string = lines.next().unwrap().unwrap();
    assert!(is_valid_binary_string(&initial_binary_string), "Initial binary string must consist solely of '0's and '1's");

    // Initialize the result based on the initial binary string
    let mut result = binary_string_to_integer(&initial_binary_string);

    // Process each subsequent binary string
    for _ in 0..n {
        let binary_string = lines.next().unwrap().unwrap();
        assert!(is_valid_binary_string(&binary_string), "Subsequent binary strings must consist solely of '0's and '1's");

        // Perform bitwise operations (AND in this case)
        result &= binary_string_to_integer(&binary_string);
    }

    // Ensure the result is within the specified bounds
    result %= 998244353;

    // Print the final result
    println!("{}", result);
}

// Function to check if a binary string is valid
fn is_valid_binary_string(s: &str) -> bool {
    s.chars().all(|c| c == '0' || c == '1')
}

// Function to convert a binary string to an integer
fn binary_string_to_integer(s: &str) -> usize {
    usize::from_str_radix(s, 2).expect("Failed to convert binary string to integer")
}
