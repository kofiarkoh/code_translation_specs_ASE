use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first integer n
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut t = vec![0; n];

    // Read the n integers into array t
    for i in 0..n {
        t[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }

    // Read the second integer m
    let m: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut p = vec![0; m];
    let mut x = vec![0; m];

    // Read the m pairs of integers into arrays p and x
    for i in 0..m {
        p[i] = lines.next().unwrap().unwrap().parse().unwrap();
        x[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }

    // Calculate the sum of the elements in t
    let sum: i32 = t.iter().sum();

    // Output the results
    for i in 0..m {
        // Adjusting for 0-based indexing in Rust
        println!("{}", sum - t[p[i] as usize - 1] + x[i]);
    }
}
