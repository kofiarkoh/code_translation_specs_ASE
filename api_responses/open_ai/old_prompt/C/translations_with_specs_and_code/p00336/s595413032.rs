// AOJ 341 Repeated Spell
// Translated from C to Rust

use std::io::{self, BufRead};

const M: i64 = 1_000_000_007;

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().collect::<Result<Vec<_>, _>>().unwrap();
    
    let t = input[0].as_bytes();
    let b = input[1].as_bytes();
    
    let st = t.len();
    let sb = b.len();
    
    // Initialize the 2D array `a` with dimensions (st + 1) x (sb + 1)
    let mut a = vec![vec![0; sb + 1]; st + 1];
    
    // Base case: There is one way to form the empty subsequence
    for i in 0..=st {
        a[i][0] = 1;
    }
    
    // Fill the DP table
    for i in 1..=st {
        for j in 0..sb {
            a[i][j + 1] = a[i - 1][j + 1] % M; // Carry forward the previous count
            if t[i - 1] == b[j] {
                a[i][j + 1] = (a[i][j + 1] + a[i - 1][j]) % M; // Add the count from the previous character
            }
        }
    }
    
    // Output the result
    println!("{}", a[st][sb]);
}
