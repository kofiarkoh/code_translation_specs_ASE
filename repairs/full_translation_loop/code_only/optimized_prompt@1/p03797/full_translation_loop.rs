use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut iter = input.split_whitespace();
    let n: i64 = iter.next().unwrap().parse().expect("Failed to parse n");
    let m: i64 = iter.next().unwrap().parse().expect("Failed to parse m");

    let sum: i64;
    if m == 1 {
        sum = 0;
    } else if (m / 2 - n) < 0 {
        sum = m / 2;
    } else if (m / 2 - n) < 2 {
        sum = n;
    } else {
        sum = n + (m / 2 - n) / 2;
    }

    // Print the result, ensuring the format matches the C code
    print!("{}", sum);
    io::stdout().flush().expect("Failed to flush stdout");
}
