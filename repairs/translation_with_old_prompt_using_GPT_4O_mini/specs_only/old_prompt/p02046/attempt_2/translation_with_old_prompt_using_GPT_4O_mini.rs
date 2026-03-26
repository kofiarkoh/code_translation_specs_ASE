use std::io::{self, BufRead};

fn main() {
    // Read from standard input until EOF
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Unwrap the line to get the input string
        let line = line.unwrap();
        // Split the line into parts and parse them as integers
        let mut parts = line.split_whitespace();
        
        // Read the first integer x
        let x: i32 = parts.next().unwrap().parse().unwrap();
        // Read the second integer y
        let y: i32 = parts.next().unwrap().parse().unwrap();

        // Compute the absolute values
        let abs_x = x.abs();
        let abs_y = y.abs();

        // Compute the value t based on the absolute values and their parity
        let t = if abs_x % 2 == 0 && abs_y % 2 == 0 {
            abs_x + abs_y
        } else if abs_x % 2 != 0 && abs_y % 2 != 0 {
            abs_x * abs_y
        } else {
            abs_x + abs_y // Corrected to sum for mixed parity
        };

        // Print the computed value t
        println!("{}", t);
    }
}
