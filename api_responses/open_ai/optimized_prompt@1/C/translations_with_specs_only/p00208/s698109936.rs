use std::io::{self, BufRead};

fn main() {
    // Create a buffer to read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Array to hold octal digits, initialized to zero
    let mut octal_digits = [0; 200];

    for line in handle.lines() {
        // Read each line of input
        let input = line.expect("Failed to read line");
        let number: i32 = input.trim().parse().expect("Invalid input");

        // Check for sentinel value (0)
        if number == 0 {
            break;
        }

        // Convert the number to octal representation
        let mut n = number;
        let mut index = 0;

        // Populate the octal_digits array
        while n > 0 {
            octal_digits[index] = (n % 8) as u8; // Store octal digit
            n /= 8;
            index += 1;
        }
        // Mark the end of valid octal digits with a terminating zero
        octal_digits[index] = 0;

        // Transform the octal digits according to the specification
        for i in 0..index {
            if octal_digits[i] > 3 {
                octal_digits[i] += 1; // Increment if greater than 3
            }
            if octal_digits[i] > 5 {
                octal_digits[i] += 1; // Increment again if greater than 5
            }
        }

        // Output the transformed octal representation in reverse order
        for i in (0..index).rev() {
            print!("{}", octal_digits[i]);
        }
        println!(); // Newline after each output
    }
}
