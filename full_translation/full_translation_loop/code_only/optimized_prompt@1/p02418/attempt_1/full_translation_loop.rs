use std::io::{self, Write};

fn main() {
    let mut str1 = String::new();
    let mut str2 = String::new();
    
    // Read input for str1
    io::stdin().read_line(&mut str1).expect("Failed to read str1");
    // Read input for str2
    io::stdin().read_line(&mut str2).expect("Failed to read str2");

    // Trim the newline characters from the input
    let str1 = str1.trim();
    let str2 = str2.trim();

    // Create a new string to hold the concatenated result
    let mut str = String::with_capacity(300);
    
    // Copy str1 to str
    str.push_str(str1);
    // Concatenate str1 to str again
    str.push_str(str1);

    // Check if str2 is a substring of str
    if str.contains(str2) {
        println!("Yes");
    } else {
        println!("No");
    }
}
