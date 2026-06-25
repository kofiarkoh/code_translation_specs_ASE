use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse two i64 values (N, K).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // Per spec: truncating division (for N >= 0 and K > 0 this matches floor).
    let x = n / k;
    let x3 = x * x * x;

    let ans = if k % 2 != 0 {
        x3
    } else {
        let y = (n + k / 2) / k;
        let y3 = y * y * y;
        x3 + y3
    };

    // Exact output formatting: single integer + newline.
    println!("{ans}");
}
