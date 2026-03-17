use std::io::{self, BufRead};

fn main() {
    // Create a buffer to read input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Initialize counts for 'o' and 'x'
    let mut count_o = 0;
    let mut count_x = 0;

    // Count occurrences of 'o' and 'x'
    for (i, c) in input.chars().enumerate() {
        if c == 'o' {
            count_o += 1;
        } else if c == 'x' {
            count_x += 1;
        }
    }

    // Determine which character to output based on counts
    if count_x >= count_o {
        println!("x");
    } else {
        println!("o");
    }
}
