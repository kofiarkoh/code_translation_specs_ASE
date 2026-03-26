use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut iter = input.split_whitespace();
    let l: i32 = iter.next().unwrap().parse().expect("Failed to parse L");
    let r: i32 = iter.next().unwrap().parse().expect("Failed to parse R");
    let d: i32 = iter.next().unwrap().parse().expect("Failed to parse d");

    // Calculate the number of multiples of d in the range [L, R]
    let result = r / d - (l - 1) / d;

    // Print the result followed by a newline
    print!("{}\n", result);
    io::stdout().flush().expect("Failed to flush stdout");
}
