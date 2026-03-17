use std::io::{self, BufRead};

fn main() {
    // Create a buffer to read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Iterate over each line of input
    for line in handle.lines() {
        // Unwrap the line to get the string
        let line = line.unwrap();
        
        // Parse the input as an integer
        let number: u32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // If parsing fails, skip to the next line (undefined behavior for invalid input)
        };

        // Check for the sentinel value (0)
        if number == 0 {
            break; // Exit the loop if sentinel value is encountered
        }

        // Convert the number to octal representation
        let mut octal_digits = [0; 11]; // Sufficient space for octal representation of u32
        let mut temp = number;
        let mut index = 0;

        // Convert to octal
        while temp > 0 {
            octal_digits[index] = (temp % 8) as u8; // Store the octal digit
            temp /= 8; // Divide by 8 to get the next digit
            index += 1;
        }

        // Modify the octal digits based on the specified rules
        for i in 0..index {
            if octal_digits[i] > 3 {
                octal_digits[i] += 1; // Increment if greater than 3
            }
            if octal_digits[i] > 5 {
                octal_digits[i] += 1; // Increment again if greater than 5
            }
        }

        // Output the modified octal representation
        // We need to print in reverse order since we filled the array from least significant to most
        for i in (0..index).rev() {
            print!("{}", octal_digits[i]);
        }
        println!(); // Newline after each output
    }
}
