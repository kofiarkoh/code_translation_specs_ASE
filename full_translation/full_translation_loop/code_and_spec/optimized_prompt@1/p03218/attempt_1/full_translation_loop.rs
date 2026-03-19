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

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a: Vec<i64> = Vec::with_capacity(n);

    // Read the elements
    for _ in 0..n {
        let value: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        a.push(value);
    }

    // Sort the array
    a.sort_unstable();

    // Compute the product of GCDs
    let mut ans: i64 = 1;
    for (i, &value) in a.iter().enumerate() {
        ans = (ans * gcd(value, i as i64)) % P;
    }

    // Output the result
    println!("{}", ans);
}
