use std::io::{self, BufRead};
use std::cmp::Ordering;

const P: i64 = 998244353;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read N
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    assert!(n > 0, "N must be greater than 0");

    // Read A
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let value: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        a.push(value);
    }

    // Sort A
    a.sort();

    // Compute the product of GCDs
    let mut ans: i64 = 1;
    for i in 0..n {
        ans = (ans * gcd(a[i], i as i64)) % P;
    }

    // Output the result
    println!("{}", ans);
}
