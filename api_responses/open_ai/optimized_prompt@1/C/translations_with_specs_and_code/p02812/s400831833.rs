fn main() {
    use std::io::{self, Write};

    let mut input = String::new();
    
    // Read the length of the string
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before reading
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for length");

    // Preconditions check
    if n < 3 {
        println!("0");
        return;
    }

    // Read the string
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let string = input.trim();

    // Ensure the string length matches the specified length
    if string.len() < n {
        // Behavior is not defined as per the spec, but we can handle it gracefully
        println!("0");
        return;
    }

    let mut count = 0;
    let mut i = 0;

    // Count non-overlapping occurrences of "ABC"
    while i <= n - 3 {
        if &string[i..i + 3] == "ABC" {
            count += 1;
            i += 3; // Move past the found substring
        } else {
            i += 1; // Move to the next character
        }
    }

    println!("{}", count);
}
