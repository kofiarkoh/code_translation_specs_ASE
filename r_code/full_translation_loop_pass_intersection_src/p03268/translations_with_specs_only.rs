use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // Per specification: C long long semantics with truncating division.
    // Under preconditions (N >= 0, K != 0, and no overflow), this matches floor division.
    let x: i64 = n / k;

    let x3: i64 = x * x * x;

    let ans: i64 = if (k & 1) != 0 {
        x3
    } else {
        // K even: y = floor((N + K/2)/K) under stated assumptions.
        let y: i64 = (n + k / 2) / k;
        let y3: i64 = y * y * y;
        x3 + y3
    };

    println!("{ans}");
}
