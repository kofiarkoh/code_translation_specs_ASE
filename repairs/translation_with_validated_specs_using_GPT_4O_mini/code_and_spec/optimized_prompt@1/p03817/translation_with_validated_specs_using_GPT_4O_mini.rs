use std::io::{self, BufRead};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Parse the input as a long long integer (i64 in Rust)
    let n: i64 = input.parse().expect("Input must be a valid long long integer");

    // Calculate the result based on the specification
    let res = if n % 11 == 0 {
        0
    } else if (1..=6).contains(&(n % 11)) {
        1
    } else {
        2
    };

    let ans = n / 11 * 2 + res;

    // Print the result
    println!("{}", ans);
}
