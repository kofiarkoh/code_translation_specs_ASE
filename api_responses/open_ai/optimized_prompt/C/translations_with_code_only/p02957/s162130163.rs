use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into parts and parse them as i64 (equivalent to long int in C)
    let mut parts = input.split_whitespace();
    let a: i64 = parts.next().unwrap().parse().expect("Failed to parse a");
    let b: i64 = parts.next().unwrap().parse().expect("Failed to parse b");

    // Check if the sum is odd
    if (a + b) % 2 != 0 {
        println!("IMPOSSIBLE");
        return;
    }

    // Print the result as i32 (equivalent to %d in C)
    // Safe because (a + b) / 2 will fit in i32 if a and b are within the range of i32
    println!("{}", (a + b) / 2);
}
