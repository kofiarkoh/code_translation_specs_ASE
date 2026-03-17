// AOJ 341 Repeated Spell
// 2017.10.26

use std::io::{self, BufRead};

const M: i64 = 1_000_000_007;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = handle.lines();

    // Read the two strings
    let t = input.next().unwrap().unwrap();
    let b = input.next().unwrap().unwrap();
    
    let st = t.len();
    let sb = b.len();
    
    // Initialize the 2D array with zeros
    let mut a = vec![vec![0; 1002]; 1002];

    // Set the first column to 1
    for i in 0..=st {
        a[i][0] = 1;
    }

    // Fill the array according to the logic in the C code
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
