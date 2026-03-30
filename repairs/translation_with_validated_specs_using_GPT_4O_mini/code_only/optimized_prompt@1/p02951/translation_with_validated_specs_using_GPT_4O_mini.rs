use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut nums = input.split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse integer"))
        .collect::<Vec<i32>>();

    let (mut a, mut b, mut c) = (nums[0], nums[1], nums[2]);
    
    a -= b; // Subtract b from a

    while a > 0 {
        a -= 1; // Decrement a
        c -= 1; // Decrement c
        if c == 0 {
            break; // Exit loop if c reaches 0
        }
    }

    // Print the result, ensuring to match the output format
    println!("{}", c);
}
