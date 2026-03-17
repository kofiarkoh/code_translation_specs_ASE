use std::io::{self, Write};

fn main() {
    let mut n = [0; 200]; // Array to hold the octal digits
    loop {
        let mut input = String::new();
        io::stdout().flush().unwrap(); // Ensure prompt is displayed before reading input
        io::stdin().read_line(&mut input).unwrap();
        let a: i32 = input.trim().parse().unwrap(); // Read and parse input

        if a == 0 {
            break; // Exit loop if input is 0
        }

        let mut b = 0; // Index for storing octal digits
        let mut a_copy = a; // Copy of a for manipulation
        while a_copy != 0 {
            n[b] = (a_copy % 8) as i32; // Store the octal digit
            a_copy /= 8; // Divide by 8 for the next digit
            b += 1; // Increment index
        }

        n[b] = 0; // Null-terminate the array (not strictly necessary in Rust)
        let mut c = 0; // Unused variable, kept for semantic preservation
        let mut i = 0; // Index for processing the array

        while i <= b {
            if n[i] > 3 {
                n[i] += 1; // Increment if greater than 3
            }
            if n[i] > 5 {
                n[i] += 1; // Increment if greater than 5
            }
            i += 1; // Increment index
        }

        for i in (0..b).rev() { // Print the digits in reverse order
            print!("{}", n[i]);
        }
        println!(); // Print newline
    }
}
