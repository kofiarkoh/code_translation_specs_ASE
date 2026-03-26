use std::io::{self, Write};

fn main() {
    let mut s = String::new();
    let mut b = [0; 256]; // Frequency array for characters
    let mut a = 0; // Result variable
    let mut c = 0; // Count of consecutive characters
    let mut l = 0; // Length of the input string

    // Read input string
    io::stdout().flush().unwrap(); // Ensure prompt is shown before input
    io::stdin().read_line(&mut s).expect("Failed to read line");
    
    // Remove newline character if present
    if let Some(pos) = s.find('\n') {
        s.truncate(pos);
    }

    // Count frequency of each character
    l = s.len() as i32; // Length of the string
    for ch in s.chars() {
        b[ch as usize] += 1;
    }

    // Process the frequency array
    while l > 0 {
        for i in 0..256 {
            if b[i] > 0 {
                a += 1; // Increment result for each unique character
                l -= 1; // Decrease length
                c += 1; // Increase consecutive count
                b[i] -= 1; // Decrease frequency
            } else {
                if c > 3 {
                    a -= c - 3; // Adjust result based on consecutive count
                }
                c = 0; // Reset consecutive count
            }
        }
    }

    // Print the result
    println!("{}", a);
}
