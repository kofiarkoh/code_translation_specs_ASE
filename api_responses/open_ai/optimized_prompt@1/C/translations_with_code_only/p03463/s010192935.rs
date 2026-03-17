use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input into integers
    let mut iter = input.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().expect("Failed to parse N");
    let a: i32 = iter.next().unwrap().parse().expect("Failed to parse A");
    let b: i32 = iter.next().unwrap().parse().expect("Failed to parse B");

    // Check the condition and print the result
    if (b - a) % 2 == 0 {
        print!("Alice\n");
    } else {
        print!("Borys\n");
    }
    
    // Flush stdout to ensure all output is printed
    io::stdout().flush().expect("Failed to flush stdout");
}
