use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as a long long integer (i64 in Rust)
    let n: i64 = input.trim().parse().expect("Invalid input");

    let res: i64;
    if n % 11 == 0 {
        res = 0;
    } else if n % 11 >= 1 && n % 11 <= 6 {
        res = 1;
    } else {
        res = 2;
    }

    let ans = n / 11 * 2 + res;

    // Print the result
    println!("{}", ans);
}
