fn souti(n: usize, p: &[char]) -> char {
    // The function evaluates the negation of the logical AND operation
    // across the boolean values represented by 'T' and 'F'.
    let mut and_result = true; // Start with true for AND operation

    for &c in p.iter().take(n) { // Process only the first n characters
        match c {
            'T' => {}, // 'T' does not affect AND result
            'F' => and_result = false, // 'F' makes AND result false
            _ => panic!("Invalid character in input"), // Handle invalid input
        }
    }

    // Negate the AND result: if AND is true, output 'F', else output 'T'
    if and_result {
        'F'
    } else {
        'T'
    }
}

fn main() {
    // Example input handling (not specified in the original spec)
    let n: usize = 3; // This should be read from input in a real scenario
    let p: Vec<char> = vec!['T', 'F', 'T']; // This should also be read from input

    if n == 0 {
        std::process::exit(1); // Exit if N is 0, as behavior is undefined
    }

    // Call the souti function and print the result
    let result = souti(n, &p);
    println!("{}", result); // Output the result followed by a newline
}
