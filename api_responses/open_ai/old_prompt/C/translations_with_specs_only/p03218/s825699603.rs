use std::io::{self, BufRead};
use std::cmp::min;

const MOD: i64 = 998244353;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    // Read the number of elements
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();
    
    // Read N
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for N");
    
    // Ensure N is greater than 0
    if n == 0 {
        panic!("N must be greater than 0");
    }

    // Read the array A
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let value: i64 = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for array element");
        a.push(value);
    }

    // Sort the array A
    a.sort();

    // Initialize the product variable
    let mut ans: i64 = 1;

    // Compute the product of GCDs
    for i in 0..n {
        let gcd_value = gcd(a[i], i as i64);
        ans = (ans * gcd_value) % MOD;
    }

    // Output the result
    println!("{}", ans);
}
