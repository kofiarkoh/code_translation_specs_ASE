use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // Preconditions (per spec): N >= 0, K != 0, and all arithmetic fits in i64.
    // We intentionally do not add extra error handling or overflow checks.
    let a = n / k;
    let mut ans = a * a * a;

    if k % 2 == 0 {
        let b = (n + k / 2) / k;
        ans += b * b * b;
    }

    println!("{}", ans);
}
