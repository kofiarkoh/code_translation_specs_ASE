use std::io::{self, BufRead};

const M: i64 = 998244353;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line to get the count of subsequent integers
    let first_line = lines.next().unwrap().unwrap();
    let count: usize = first_line.parse().unwrap(); // Assumes valid integer input

    let mut d = vec![0; count + 1]; // Array d
    let mut e = vec![0; count + 1]; // Array e
    let mut r = 1; // Result variable
    let mut s = 0; // Cumulative sum

    for _ in 0..count {
        let line = lines.next().unwrap().unwrap();
        let a: usize = line.parse().unwrap(); // Assumes valid integer input
        s += a;

        // Ensure the arrays can accommodate the new indices
        if s + 1 > d.len() {
            d.resize(s + 1, 0);
            e.resize(s + 1, 0);
        }

        // Update arrays d and e
        for t in 0..=s {
            if t + a < d.len() {
                d[t + a] = (d[t + a] + d[t] * 2 % M) % M; // Update d
                e[t + a] = (e[t + a] + e[t]) % M; // Update e
            }
        }

        r = r * 3 % M; // Update result variable r
    }

    // Final result computation
    for t in (0..=(s / 2)).rev() {
        r = (r - d[t] * 3) % M; // Adjust r based on d
    }
    r = (r + e[s / 2]) % M; // Adjust r based on e

    // Output the final result
    println!("{}", r);
}
