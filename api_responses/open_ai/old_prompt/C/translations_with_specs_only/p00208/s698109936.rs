use std::io::{self, Write};

fn main() {
    // Fixed-size array to hold octal digits
    let mut n = [0; 200];
    
    loop {
        // Read input from the user
        let mut input = String::new();
        print!("Enter a decimal integer (0 to exit): ");
        io::stdout().flush().unwrap(); // Ensure prompt is displayed
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        // Parse the input as a non-negative integer
        let a: i32 = match input.trim().parse() {
            Ok(num) if num >= 0 => num,
            _ => continue, // Ignore invalid input and continue the loop
        };

        // Terminate the program if the input is 0
        if a == 0 {
            break;
        }

        // Convert decimal integer to octal representation
        let mut b = 0; // Track the number of octal digits
        let mut value = a;

        // Convert to octal
        while value > 0 {
            n[b] = value % 8; // Store the octal digit
            value /= 8; // Reduce the value
            b += 1; // Increment the digit count
        }

        // Output the modified octal representation
        // The digits are stored in reverse order, so we need to print them in reverse
        for i in (0..b).rev() {
            print!("{}", n[i]);
        }
        println!(); // New line after each output
    }
}
