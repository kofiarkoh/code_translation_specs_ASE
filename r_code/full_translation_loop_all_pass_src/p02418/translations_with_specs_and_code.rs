use std::io::{self, Write};

fn main() {
    let mut str1 = String::new();
    let mut str2 = String::new();

    // Read the first input string
    io::stdin().read_line(&mut str1).expect("Failed to read line");
    // Read the second input string
    io::stdin().read_line(&mut str2).expect("Failed to read line");

    // Trim whitespace and ensure the strings do not exceed the specified lengths
    let str1 = str1.trim();
    let str2 = str2.trim();

    // Check preconditions
    if str1.len() > 100 || str2.len() > 100 || str1.is_empty() || str2.is_empty() {
        // Behavior is not defined for invalid inputs as per the specification
        return;
    }

    // Concatenate str1 with itself
    let concatenated = format!("{}{}", str1, str1);

    // Check if str2 is a substring of the concatenated string
    if concatenated.contains(str2) {
        println!("Yes");
    } else {
        println!("No");
    }
}
