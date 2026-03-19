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

fn compare(a: &i64, b: &i64) -> Ordering {
    a.cmp(b)
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse N");
    let mut a: Vec<i64> = Vec::with_capacity(n);

    // Read the elements into the vector
    for _ in 0..n {
        let value: i64 = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse value");
        a.push(value);
    }

    // Sort the vector
    a.sort_by(compare);

    let mut ans = 1;
    for (i, &value) in a.iter().enumerate() {
        ans = (ans * gcd(value, i as i64)) % P;
    }

    // Print the result
    println!("{}", ans);
}
