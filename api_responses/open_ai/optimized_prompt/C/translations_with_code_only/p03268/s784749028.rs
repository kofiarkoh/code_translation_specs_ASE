use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    
    // Parse the input values as i64 to match long long in C
    let n: i64 = iter.next().unwrap().parse().unwrap();
    let k: i64 = iter.next().unwrap().parse().unwrap();

    let result = if k % 2 == 1 {
        // When K is odd
        let quotient = n / k;
        quotient * quotient * quotient
    } else {
        // When K is even
        let quotient1 = n / k;
        let quotient2 = (n + k / 2) / k;
        quotient1 * quotient1 * quotient1 + quotient2 * quotient2 * quotient2
    };

    // Print the result followed by a newline
    println!("{}", result);
    // Flush stdout to ensure the output is written immediately
    io::stdout().flush().unwrap();
}
