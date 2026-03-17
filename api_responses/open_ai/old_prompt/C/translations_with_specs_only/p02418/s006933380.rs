use std::io::{self, Write};

fn main() {
    // Read the first string
    let mut str1 = String::new();
    io::stdin().read_line(&mut str1).expect("Failed to read line");
    // Trim the newline character
    let str1 = str1.trim();

    // Read the second string
    let mut str2 = String::new();
    io::stdin().read_line(&mut str2).expect("Failed to read line");
    // Trim the newline character
    let str2 = str2.trim();

    // Check the lengths of the strings to ensure they are within the specified limits
    if str1.len() > 100 || str2.len() > 100 {
        // If either string exceeds the limit, we do not define behavior, but we can safely exit
        return;
    }

    // Handle the edge case where both strings are empty
    if str1.is_empty() && str2.is_empty() {
        println!("Yes");
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
