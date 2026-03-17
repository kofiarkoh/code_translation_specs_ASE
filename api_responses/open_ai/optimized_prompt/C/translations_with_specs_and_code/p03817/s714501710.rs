use std::io::{self, Write};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as a valid integer
    let n: i64 = input.trim().parse().expect("Input must be a valid integer");

    // Initialize the result variable
    let res: i64;

    // Determine the value of res based on the remainder of n when divided by 11
    if n % 11 == 0 {
        res = 0;
    } else if (1..=6).contains(&(n % 11)) {
        res = 1;
    } else {
        res = 2;
    }

    // Calculate the final answer
    let ans = (n / 11) * 2 + res;

    // Print the result to standard output
    println!("{}", ans);
}
