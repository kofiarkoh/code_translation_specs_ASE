use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut iter = input.split_whitespace();
    let n: i64 = iter.next().unwrap().parse().expect("Invalid input for N");
    let k: i64 = iter.next().unwrap().parse().expect("Invalid input for K");

    // Check for division by zero
    if k == 0 {
        // Undefined behavior in the original C code; we will not handle this case explicitly.
        // The program will simply panic if k is zero.
        panic!("Division by zero error");
    }

    let quotient = n / k;
    if k % 2 == 1 {
        // K is odd
        println!("{}", quotient.pow(3));
    } else {
        // K is even
        let modified_n = n + k / 2;
        let modified_quotient = modified_n / k;
        println!("{}", quotient.pow(3) + modified_quotient.pow(3));
    }
}
