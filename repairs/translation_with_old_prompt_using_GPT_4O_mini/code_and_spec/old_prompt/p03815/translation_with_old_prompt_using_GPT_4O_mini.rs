use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Parse the input as a 64-bit signed integer
    let x: i64 = input.parse().expect("Invalid input; expected a long long integer.");

    // Compute the output based on the divisibility by 11
    let result = if x % 11 == 0 {
        2 * (x / 11)
    } else if x % 11 > 6 {
        2 * (x / 11) + 2
    } else {
        2 * (x / 11) + 1
    };

    // Print the result followed by a newline
    println!("{}", result);
}
