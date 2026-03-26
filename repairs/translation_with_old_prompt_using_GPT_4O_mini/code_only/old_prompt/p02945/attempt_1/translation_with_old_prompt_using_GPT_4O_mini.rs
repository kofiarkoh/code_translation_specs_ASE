use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the two integers from the input
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().expect("Failed to parse a");
    let b: i32 = iter.next().unwrap().parse().expect("Failed to parse b");

    let c = a + b;
    let d = a - b;
    let e = a * b;

    // Initialize max to a value lower than any possible result
    let mut max = -1000;

    // Update max if c, d, or e is greater
    if max < c {
        max = c;
    }
    if max < d {
        max = d;
    }
    if max < e {
        max = e;
    }

    // Print the result
    print!("{}", max);
    io::stdout().flush().expect("Failed to flush stdout");
}
