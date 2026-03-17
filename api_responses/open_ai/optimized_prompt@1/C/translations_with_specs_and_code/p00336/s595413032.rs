// AOJ 341 Repeated Spell
// Translated from C to Rust

use std::io::{self, BufRead};

const M: i64 = 1_000_000_007;

fn main() {
    // Read input strings
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();
    
    let t = input.next().unwrap().unwrap();
    let b = input.next().unwrap().unwrap();
    
    let st = t.len();
    let sb = b.len();
    
    // Create a 2D array to store the counts of distinct subsequences
    let mut a = vec![vec![0; sb + 1]; st + 1];
    
    // Initialize the first column
    for i in 0..=st {
        a[i][0] = 1;
    }
    
    // Fill the DP table
    for i in 1..=st {
        for j in 0..sb {
            a[i][j + 1] = a[i - 1][j + 1] % M; // Carry forward the previous count
            if t.as_bytes()[i - 1] == b.as_bytes()[j] {
                a[i][j + 1] = (a[i][j + 1] + a[i - 1][j]) % M; // Update count if characters match
            }
        }
    }
    
    // Output the result
    println!("{}", a[st][sb]);
}
