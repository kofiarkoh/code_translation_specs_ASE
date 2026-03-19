// AOJ 341 Repeated Spell
// 2017.10.26

use std::io::{self, BufRead};

const M: i64 = 1_000_000_007;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the two strings
    let t = lines.next().unwrap().unwrap();
    let b = lines.next().unwrap().unwrap();
    
    let st = t.len();
    let sb = b.len();
    
    // Create a 2D vector initialized to 0
    let mut a = vec![vec![0; sb + 1]; st + 1];

    // Initialize the first column
    for i in 0..=st {
        a[i][0] = 1;
    }

    // Fill the DP table
    for i in 1..=st {
        for j in 0..sb {
            a[i][j + 1] = a[i - 1][j + 1] % M;
            if t.as_bytes()[i - 1] == b.as_bytes()[j] {
                a[i][j + 1] = (a[i][j + 1] + a[i - 1][j]) % M;
            }
        }
    }

    // Print the result
    println!("{}", a[st][sb]);
}
