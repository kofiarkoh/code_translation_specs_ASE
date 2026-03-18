use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches typical competitive-programming style; preserves semantics).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // Per spec: behavior is undefined if N is non-positive or missing/invalid.
    let n: usize = it.next().unwrap().parse().unwrap();

    let mut ans: i64 = 0;
    for _ in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        ans += x - 1;
    }

    // C prints without a trailing newline; preserve that.
    print!("{ans}");
}
