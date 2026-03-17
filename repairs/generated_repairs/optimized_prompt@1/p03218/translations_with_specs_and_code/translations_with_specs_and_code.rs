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
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.parse().unwrap_or(0),
        _ => return,
    };
    let mut a: Vec<i64> = Vec::with_capacity(n);

    // Read the elements into the vector
    for _ in 0..n {
        match lines.next() {
            Some(Ok(line)) => {
                if let Ok(value) = line.parse() {
                    a.push(value);
                } else {
                    return; // Handle parse error gracefully
                }
            },
            _ => return, // Handle missing lines gracefully
        }
    }

    // Sort the vector
    a.sort_unstable();

    // Compute the product of GCDs
    let mut ans: i64 = 1;
    for i in 0..n {
        ans = (ans * gcd(a[i], i as i64)) % P;
    }

    // Output the result
    println!("{}", ans);
}
