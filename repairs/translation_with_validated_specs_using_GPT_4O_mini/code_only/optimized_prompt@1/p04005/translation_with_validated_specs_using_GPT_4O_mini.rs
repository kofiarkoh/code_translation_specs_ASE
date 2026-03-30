use std::io::{self, Write};

fn main() {
    let mut x = [0; 3]; // Initialize an array of 3 long integers

    // Read three long integers from stdin
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Failed to read line");
    let values: Vec<&str> = input.split_whitespace().collect();
    for i in 0..3 {
        x[i] = values[i].parse::<i64>().expect("Failed to parse input");
    }

    // Check if any of the numbers are even
    if x[0] % 2 == 0 || x[1] % 2 == 0 || x[2] % 2 == 0 {
        println!("0");
    } else {
        let mut max = 0;
        let mut m = 0;

        // Find the index of the maximum value
        for i in 0..3 {
            if max < x[i] {
                max = x[i];
                m = i;
            }
        }

        // Set the maximum value's index to 1
        x[m] = 1;

        // Print the product of the three numbers
        let product = x[0] * x[1] * x[2];
        print!("{}", product);
    }
}
