use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the threshold `n` and the number of pairs `m`
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: i32 = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // Initialize arrays for the pairs
    let mut a: Vec<i32> = Vec::with_capacity(m);
    let mut b: Vec<i32> = Vec::with_capacity(m);

    // Read the pairs of integers
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut pair_parts = line.split_whitespace();
        let a_i: i32 = pair_parts.next().unwrap().parse().unwrap();
        let b_i: i32 = pair_parts.next().unwrap().parse().unwrap();
        a.push(a_i);
        b.push(b_i);
    }

    // Modify the pairs based on the threshold `n`
    let mut modified_values: Vec<i32> = Vec::new();
    for i in 0..m {
        if a[i] > n {
            modified_values.push(b[i]);
        }
    }

    // Sort the modified values
    modified_values.sort();

    // Compute the sum of selected values
    let count: i32 = modified_values.iter().sum();

    // Print the result
    println!("{}", count);
}
