use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    
    let n: i64 = iter.next().unwrap().parse().unwrap();
    let k: i64 = iter.next().unwrap().parse().unwrap();

    if k % 2 == 1 {
        // When K is odd, calculate (N / K) cubed
        let result = (n / k).pow(3);
        println!("{}", result);
    } else {
        // When K is even, calculate two terms and sum them
        let term1 = (n / k).pow(3);
        let term2 = ((n + k / 2) / k).pow(3);
        let result = term1 + term2;
        println!("{}", result);
    }

    // Flush stdout to ensure all output is written
    io::stdout().flush().unwrap();
}
